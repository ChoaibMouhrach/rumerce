use std::collections::HashMap;

use axum::{
    extract::{Query, State},
    http::{header, StatusCode},
    response::IntoResponse,
    Extension, Json,
};
use chrono::{Duration, Utc};
use log::error;
use resend_rs::types::CreateEmailBaseOptions;
use serde::Serialize;
use url::Url;
use uuid::Uuid;

use crate::{
    models::{role::Role, session::PopulatedSession, user::User},
    services,
    utils::constants::{ROLES, SESSION_COOKIE_NAME},
    validations::{
        auth::{SignInSchema, StoreSessionSchema},
        magic_tokens::StoreMagicTokenSchema,
        user::StoreUserSchema,
        ValidatedForm,
    },
    AppState,
};

pub async fn sign_in(
    State(state): State<AppState>,
    ValidatedForm(input): ValidatedForm<SignInSchema>,
) -> impl IntoResponse {
    let mut connection = match state.db.acquire().await {
        Ok(connection) => connection,
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    let user = match services::user::find_by_email(&input.email, &mut connection).await {
        Ok(user) => user,
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    let user = match user {
        Some(user) => user,
        None => {
            let role = match services::role::find_by_name(ROLES.member, &mut connection).await {
                Ok(Some(role)) => role,
                Ok(None) => {
                    error!("{} role not found", ROLES.member);
                    return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
                }
                Err(err) => {
                    error!("{err}");
                    return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
                }
            };

            let new_user = services::user::insert(
                &StoreUserSchema {
                    name: None,
                    role_id: role.id,
                    email: input.email.clone(),
                },
                &mut connection,
            )
            .await;

            match new_user {
                Ok(user) => user,
                Err(err) => {
                    error!("{err}");
                    return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
                }
            }
        }
    };

    let token = Uuid::new_v4();

    let token = services::magic_tokens::insert(
        &StoreMagicTokenSchema {
            token,
            user_id: user.user.id,
            expires_at: Utc::now().naive_utc() + Duration::hours(1),
        },
        &mut connection,
    )
    .await;

    let token = match token {
        Ok(token) => token,
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    let mut url = match Url::parse(&state.env.client_url) {
        Ok(url) => url,
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    // set the path
    url.set_path("/auth");

    // set the token
    url.query_pairs_mut()
        .append_pair("token", &token.token.token.to_string());

    // prepare the url
    let url = url.to_string();

    let email = CreateEmailBaseOptions::new(
        format!("{}@{}", "auth", state.env.resend_domain),
        vec![&input.email],
        "Authentication",
    )
    .with_html(&format!(r#"<a href="{}" >Sign In</a>"#, url));

    if let Err(err) = state.mailer.mail(email).await {
        error!("{err}");
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    }

    ().into_response()
}

pub async fn auth(
    Query(params): Query<HashMap<String, String>>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let token = match params.get("token") {
        Some(token) => token,
        None => {
            return (StatusCode::BAD_REQUEST, Json("Token not provided")).into_response();
        }
    };

    let token = match Uuid::parse_str(token) {
        Ok(token) => token,
        Err(_) => {
            return (StatusCode::BAD_REQUEST, Json("Invalid token")).into_response();
        }
    };

    let mut connection = match state.db.acquire().await {
        Ok(connection) => connection,
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    let token = services::magic_tokens::find_by_token(&token, &mut connection).await;

    let token = match token {
        Ok(Some(token)) => token,
        Ok(None) => {
            return (StatusCode::BAD_REQUEST, Json("Token not found")).into_response();
        }
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    if let Err(err) = services::magic_tokens::destroy(&token.token.id, &mut connection).await {
        error!("{err}");
        return (StatusCode::INTERNAL_SERVER_ERROR, Json("Token expired")).into_response();
    };

    if Utc::now().naive_utc() > token.token.created_at {
        return (StatusCode::BAD_REQUEST, Json("Token expired")).into_response();
    }

    let session = services::sessions::insert(
        StoreSessionSchema {
            user_id: token.user.id,
            session: Uuid::new_v4(),
        },
        &mut connection,
    )
    .await;

    let session = match session {
        Ok(session) => session,
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    (
        [(
            header::SET_COOKIE,
            format!(
                "{}={}; Path=/; HttpOnly; Secure; SameSite=Strict; Max-Age=2592000",
                SESSION_COOKIE_NAME, session.session
            ),
        )],
        Json(session.session),
    )
        .into_response()
}

pub async fn sign_out(
    Extension(auth): Extension<PopulatedSession>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let mut connection = match state.db.acquire().await {
        Ok(connection) => connection,
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    if let Err(err) = services::sessions::delete(&auth.session.id, &mut connection).await {
        error!("{err}");
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    }

    ([(
        header::SET_COOKIE,
        format!(
            "{}=; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT",
            SESSION_COOKIE_NAME
        ),
    )])
    .into_response()
}

#[derive(Serialize)]
struct Profile {
    pub user: User,
    pub role: Role,
}

pub async fn profile(Extension(auth): Extension<PopulatedSession>) -> impl IntoResponse {
    Json(Profile {
        user: auth.user,
        role: auth.role,
    })
    .into_response()
}

use std::collections::HashMap;

use axum::{
    extract::{Query, State},
    http::{
        header::{self},
        StatusCode,
    },
    response::IntoResponse,
    Extension, Json,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use log::error;
use resend_rs::types::CreateEmailBaseOptions;
use serde::Serialize;
use url::Url;
use uuid::Uuid;

use crate::{
    models::{role::Role, session::PopulatedSession, user::User},
    services,
    utils::constants::ROLES,
    validations::{
        auth::{SignInSchema, SignInTokenSchema, StoreSessionSchema},
        user::StoreUserSchema,
    },
    AppState,
};

pub async fn sign_in(
    State(state): State<AppState>,
    Json(input): Json<SignInSchema>,
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
                    //TODO: Inv later
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

    let token = encode(
        &Header::default(),
        &SignInTokenSchema {
            user_id: user.user.id,
            exp: (Utc::now() + Duration::minutes(60)).timestamp(),
        },
        &EncodingKey::from_secret(state.env.app_secret.as_bytes()),
    );

    let token = match token {
        Ok(token) => token,
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    let mut url = match Url::parse(&state.env.app_url) {
        Ok(url) => url,
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    // set the path
    url.set_path("/auth");

    // set the token
    url.query_pairs_mut().append_pair("token", &token);

    // prepare the url
    let url = url.to_string();

    let email = CreateEmailBaseOptions::new(
        format!("{}@{}", "auth", state.env.resend_domain),
        vec![&input.email],
        "Authentication",
    )
    .with_html(&format!(r#"<a href="{}" >Sign In</a>"#, url));

    if let Err(err) = state.resend.emails.send(email).await {
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
            return (StatusCode::BAD_REQUEST, Json("Token not found")).into_response();
        }
    };

    let token = decode::<SignInTokenSchema>(
        token,
        &DecodingKey::from_secret(state.env.app_secret.as_bytes()),
        &Validation::default(),
    );

    let token = match token {
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

    let user = match services::user::find(&token.claims.user_id, &mut connection).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return (StatusCode::NOT_FOUND, Json("User not found")).into_response();
        }
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    let session = services::sessions::insert(
        StoreSessionSchema {
            user_id: user.user.id,
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

    ([(
        header::SET_COOKIE,
        format!("{}={}", "session", session.session.to_string()),
    )])
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

    ().into_response()
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

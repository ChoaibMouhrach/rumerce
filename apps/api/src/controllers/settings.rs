use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use log::error;

use crate::{
    services,
    utils::constants::{ROLES, SETTINGS},
    validations::{
        role::StoreRoleSchema,
        settings::{SetupSchema, StoreSettingsSchema},
        user::StoreUserSchema,
    },
    AppState,
};

pub async fn setup_check(State(state): State<AppState>) -> impl IntoResponse {
    let mut connection = match state.db.acquire().await {
        Ok(connection) => connection,
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    let setup = services::settings::find_by_key(SETTINGS.setup, &mut connection).await;

    if let Ok(None) = setup {
        return (StatusCode::FAILED_DEPENDENCY).into_response();
    }

    if let Err(err) = setup {
        error!("{err}");
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    }

    ().into_response()
}

pub async fn setup(
    State(state): State<AppState>,
    Json(input): Json<SetupSchema>,
) -> impl IntoResponse {
    let mut connection = match state.db.acquire().await {
        Ok(connection) => connection,
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    // check if setup is already done
    let setup = services::settings::find_by_key(SETTINGS.setup, &mut connection).await;

    if let Ok(Some(_)) = setup {
        return (StatusCode::CREATED).into_response();
    }

    if let Err(err) = setup {
        error!("{err}");
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    }

    // setup roles
    let roles = services::role::insert_many(
        &vec![
            StoreRoleSchema { name: ROLES.member },
            StoreRoleSchema { name: ROLES.admin },
        ],
        &mut connection,
    )
    .await;

    let roles = match roles {
        Ok(roles) => roles,
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    // setup admin
    let user = services::user::insert(
        &StoreUserSchema {
            name: None,
            email: input.email,
            role_id: roles
                .into_iter()
                .find(|role| role.name == ROLES.admin)
                .unwrap()
                .id,
        },
        &mut connection,
    )
    .await;

    if let Err(err) = user {
        error!("{err}");
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    }

    // mark as setup complete
    let setup = services::settings::insert(
        &StoreSettingsSchema {
            key: SETTINGS.setup.to_string(),
            value: "true".to_string(),
        },
        &mut connection,
    )
    .await;

    if let Err(err) = setup {
        error!("{err}");
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    }

    (StatusCode::CREATED).into_response()
}

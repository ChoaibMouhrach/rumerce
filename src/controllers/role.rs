use crate::{
    services,
    utils::db::DB,
    validations::role::{StoreRoleSchema, UpdateRoleSchema},
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use log::error;
use uuid::Uuid;

pub async fn index(State(db): State<DB>) -> impl IntoResponse {
    let roles = match services::role::all(&db).await {
        Ok(roles) => roles,
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    Json(roles).into_response()
}

pub async fn show(Path(id): Path<Uuid>, State(db): State<DB>) -> impl IntoResponse {
    let role = match services::role::find(&id, &db).await {
        Ok(Some(role)) => role,
        Ok(None) => {
            return (StatusCode::NOT_FOUND).into_response();
        }
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    Json(role).into_response()
}

pub async fn store(State(db): State<DB>, Json(input): Json<StoreRoleSchema>) -> impl IntoResponse {
    if let Err(err) = services::role::insert(&input, &db).await {
        error!("{err}");
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    }

    (StatusCode::CREATED).into_response()
}

pub async fn update(
    Path(id): Path<Uuid>,
    State(db): State<DB>,
    Json(input): Json<UpdateRoleSchema>,
) -> impl IntoResponse {
    if let Err(err) = services::role::update(&id, &input, &db).await {
        error!("{err}");
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    };

    ().into_response()
}

pub async fn destroy(Path(id): Path<Uuid>, State(db): State<DB>) -> impl IntoResponse {
    if let Err(err) = services::role::destroy(&id, &db).await {
        error!("{err}");
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    };

    ().into_response()
}

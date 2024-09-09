use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use log::error;
use uuid::Uuid;

use crate::{
    services,
    utils::db::DB,
    validations::user::{StoreUserSchema, UpdateUserSchema},
};

pub async fn index(State(db): State<DB>) -> impl IntoResponse {
    let users = match services::user::all(&db).await {
        Ok(users) => users,
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    Json(users).into_response()
}

pub async fn show(Path(id): Path<Uuid>, State(db): State<DB>) -> impl IntoResponse {
    let user = match services::user::find(&id, &db).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    Json(user).into_response()
}

pub async fn store(State(db): State<DB>, Json(input): Json<StoreUserSchema>) -> impl IntoResponse {
    if let Err(err) = services::user::insert(&input, &db).await {
        error!("{err}");
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    }

    (StatusCode::CREATED).into_response()
}

pub async fn update(
    Path(id): Path<Uuid>,
    State(db): State<DB>,
    Json(input): Json<UpdateUserSchema>,
) -> impl IntoResponse {
    if let Err(err) = services::user::update(&id, &input, &db).await {
        error!("{err}");
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    }

    ().into_response()
}

pub async fn destroy(Path(id): Path<Uuid>, State(db): State<DB>) -> impl IntoResponse {
    if let Err(err) = services::user::destroy(&id, &db).await {
        error!("{err}");
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    }

    ().into_response()
}

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::{
    services,
    utils::db::DB,
    validations::user::{StoreUserSchema, UpdateUserSchema},
};

pub async fn index(State(db): State<DB>) -> impl IntoResponse {
    let users = services::user::all(&db).await;
    Json(users)
}

pub async fn show(Path(id): Path<Uuid>, State(db): State<DB>) -> impl IntoResponse {
    let user = services::user::find(&id, &db).await;
    Json(user)
}

pub async fn store(State(db): State<DB>, Json(input): Json<StoreUserSchema>) -> impl IntoResponse {
    services::user::insert(&input, &db).await;
}

pub async fn update(
    Path(id): Path<Uuid>,
    State(db): State<DB>,
    Json(input): Json<UpdateUserSchema>,
) -> impl IntoResponse {
    services::user::update(&id, &input, &db).await;
}

pub async fn destroy(Path(id): Path<Uuid>, State(db): State<DB>) -> impl IntoResponse {
    services::user::destroy(&id, &db).await;
}

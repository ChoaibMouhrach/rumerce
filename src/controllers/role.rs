use crate::{
    services,
    utils::db::DB,
    validations::role::{StoreRoleSchema, UpdateRoleSchema},
};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

pub async fn index(State(db): State<DB>) -> impl IntoResponse {
    let roles = services::role::all(&db).await;
    Json(roles)
}

pub async fn show(Path(id): Path<Uuid>, State(db): State<DB>) -> impl IntoResponse {
    let role = services::role::find(&id, &db).await;
    Json(role)
}

pub async fn store(State(db): State<DB>, Json(input): Json<StoreRoleSchema>) -> impl IntoResponse {
    services::role::insert(&input, &db).await;
}

pub async fn update(
    Path(id): Path<Uuid>,
    State(db): State<DB>,
    Json(input): Json<UpdateRoleSchema>,
) -> impl IntoResponse {
    services::role::update(&id, &input, &db).await;
}

pub async fn destroy(Path(id): Path<Uuid>, State(db): State<DB>) -> impl IntoResponse {
    services::role::destroy(&id, &db).await;
}

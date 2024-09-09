use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::{services, utils::db::DB, validations::warehouse::StoreWarehouseSchema};

pub async fn index(State(db): State<DB>) -> impl IntoResponse {
    let warehouses = services::warehouse::all(&db).await;
    Json(warehouses)
}

pub async fn show(Path(id): Path<Uuid>, State(db): State<DB>) -> impl IntoResponse {
    let warehouse = services::warehouse::find(&id, &db).await;
    Json(warehouse)
}

pub async fn store(
    State(db): State<DB>,
    Json(input): Json<StoreWarehouseSchema>,
) -> impl IntoResponse {
    services::warehouse::insert(&input, &db).await;
}

pub async fn update(
    Path(id): Path<Uuid>,
    State(db): State<DB>,
    Json(input): Json<StoreWarehouseSchema>,
) -> impl IntoResponse {
    services::warehouse::update(&id, &input, &db).await;
}

pub async fn destroy(Path(id): Path<Uuid>, State(db): State<DB>) -> impl IntoResponse {
    services::warehouse::delete(&id, &db).await;
}

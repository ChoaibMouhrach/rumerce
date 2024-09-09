use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use log::error;
use uuid::Uuid;

use crate::{services, utils::db::DB, validations::warehouse::StoreWarehouseSchema};

pub async fn index(State(db): State<DB>) -> impl IntoResponse {
    let warehouses = match services::warehouse::all(&db).await {
        Ok(warehouses) => warehouses,
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    Json(warehouses).into_response()
}

pub async fn show(Path(id): Path<Uuid>, State(db): State<DB>) -> impl IntoResponse {
    let warehouse = match services::warehouse::find(&id, &db).await {
        Ok(Some(warehouse)) => warehouse,
        Ok(None) => {
            return (StatusCode::NOT_FOUND).into_response();
        }
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    Json(warehouse).into_response()
}

pub async fn store(
    State(db): State<DB>,
    Json(input): Json<StoreWarehouseSchema>,
) -> impl IntoResponse {
    if let Err(err) = services::warehouse::insert(&input, &db).await {
        error!("{err}");
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    };

    (StatusCode::CREATED).into_response()
}

pub async fn update(
    Path(id): Path<Uuid>,
    State(db): State<DB>,
    Json(input): Json<StoreWarehouseSchema>,
) -> impl IntoResponse {
    if let Err(err) = services::warehouse::update(&id, &input, &db).await {
        error!("{err}");
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    };

    ().into_response()
}

pub async fn destroy(Path(id): Path<Uuid>, State(db): State<DB>) -> impl IntoResponse {
    if let Err(err) = services::warehouse::delete(&id, &db).await {
        error!("{err}");
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    };

    ().into_response()
}

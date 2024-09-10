use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use log::error;
use uuid::Uuid;

use crate::{services, validations::warehouse::StoreWarehouseSchema, AppState};

pub async fn index(State(state): State<AppState>) -> impl IntoResponse {
    let mut connection = match state.db.acquire().await {
        Ok(connection) => connection,
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    let warehouses = match services::warehouse::all(&mut connection).await {
        Ok(warehouses) => warehouses,
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    Json(warehouses).into_response()
}

pub async fn show(Path(id): Path<Uuid>, State(state): State<AppState>) -> impl IntoResponse {
    let mut connection = match state.db.acquire().await {
        Ok(connection) => connection,
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    let warehouse = match services::warehouse::find(&id, &mut connection).await {
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
    State(state): State<AppState>,
    Json(input): Json<StoreWarehouseSchema>,
) -> impl IntoResponse {
    let mut connection = match state.db.acquire().await {
        Ok(connection) => connection,
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    if let Err(err) = services::warehouse::insert(&input, &mut connection).await {
        error!("{err}");
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    };

    (StatusCode::CREATED).into_response()
}

pub async fn update(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    Json(input): Json<StoreWarehouseSchema>,
) -> impl IntoResponse {
    let mut connection = match state.db.acquire().await {
        Ok(connection) => connection,
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    if let Err(err) = services::warehouse::update(&id, &input, &mut connection).await {
        error!("{err}");
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    };

    ().into_response()
}

pub async fn destroy(Path(id): Path<Uuid>, State(state): State<AppState>) -> impl IntoResponse {
    let mut connection = match state.db.acquire().await {
        Ok(connection) => connection,
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    if let Err(err) = services::warehouse::delete(&id, &mut connection).await {
        error!("{err}");
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    };

    ().into_response()
}

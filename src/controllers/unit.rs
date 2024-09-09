use crate::{
    services,
    utils::db::DB,
    validations::unit::{StoreUnitSchema, UpdateUnitSchema},
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
    let units = match services::unit::all(&db).await {
        Ok(units) => units,
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    Json(units).into_response()
}

pub async fn show(Path(id): Path<Uuid>, State(db): State<DB>) -> impl IntoResponse {
    let unit = match services::unit::find(&id, &db).await {
        Ok(Some(unit)) => unit,
        Ok(None) => {
            return (StatusCode::NOT_FOUND).into_response();
        }
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    Json(unit).into_response()
}

pub async fn store(State(db): State<DB>, Json(input): Json<StoreUnitSchema>) -> impl IntoResponse {
    if let Err(err) = services::unit::insert(&input, &db).await {
        error!("{err}");
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    };

    (StatusCode::CREATED).into_response()
}

pub async fn update(
    Path(id): Path<Uuid>,
    State(db): State<DB>,
    Json(input): Json<UpdateUnitSchema>,
) -> impl IntoResponse {
    if let Err(err) = services::unit::update(&id, &input, &db).await {
        error!("{err}");
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    };

    ().into_response()
}

pub async fn destroy(Path(id): Path<Uuid>, State(db): State<DB>) -> impl IntoResponse {
    if let Err(err) = services::unit::destroy(&id, &db).await {
        error!("{err}");
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    };

    ().into_response()
}

use crate::{
    services,
    utils::db::DB,
    validations::unit::{StoreUnitSchema, UpdateUnitSchema},
};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

pub async fn index(State(db): State<DB>) -> impl IntoResponse {
    let units = services::unit::all(&db).await;
    Json(units)
}

pub async fn show(Path(id): Path<Uuid>, State(db): State<DB>) -> impl IntoResponse {
    let unit = services::unit::find(&id, &db).await.unwrap();
    Json(unit)
}

pub async fn store(State(db): State<DB>, Json(input): Json<StoreUnitSchema>) -> impl IntoResponse {
    services::unit::insert(&input, &db).await;
}

pub async fn update(
    Path(id): Path<Uuid>,
    State(db): State<DB>,
    Json(input): Json<UpdateUnitSchema>,
) {
    services::unit::update(&id, &input, &db).await;
}

pub async fn destroy(Path(id): Path<Uuid>, State(db): State<DB>) {
    services::unit::destroy(&id, &db).await;
}

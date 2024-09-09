use crate::{
    services,
    utils::db::DB,
    validations::category::{StoreCategorySchema, UpdateCategorySchema},
};

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

pub async fn index(State(db): State<DB>) -> impl IntoResponse {
    let categories = services::category::all(&db).await;
    Json(categories)
}

pub async fn show(Path(id): Path<Uuid>, State(db): State<DB>) -> impl IntoResponse {
    let category = services::category::find(&id, &db).await.unwrap();
    Json(category)
}

pub async fn store(
    State(db): State<DB>,
    Json(input): Json<StoreCategorySchema>,
) -> impl IntoResponse {
    services::category::insert(&input, &db).await;
}

pub async fn update(
    Path(id): Path<Uuid>,
    State(db): State<DB>,
    Json(input): Json<UpdateCategorySchema>,
) {
    services::category::update(&id, &input, &db).await;
}

pub async fn destroy(Path(id): Path<Uuid>, State(db): State<DB>) {
    services::category::destroy(&id, &db).await;
}

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::{services, utils::db::DB, validations::product::StoreProductSchema};

pub async fn index(State(db): State<DB>) -> impl IntoResponse {
    let products = services::product::all(&db).await;
    Json(products)
}

pub async fn show(Path(id): Path<Uuid>, State(db): State<DB>) -> impl IntoResponse {
    let product = services::product::find(&id, &db).await;
    Json(product)
}

pub async fn store(
    State(db): State<DB>,
    Json(input): Json<StoreProductSchema>,
) -> impl IntoResponse {
    let product = services::product::insert(&input, &db).await;
    product.attach_variants(&input.variants, &db).await;
}

pub async fn update(
    Path(id): Path<Uuid>,
    State(db): State<DB>,
    Json(input): Json<StoreProductSchema>,
) -> impl IntoResponse {
    let product = match services::product::find(&id, &db).await {
        Some(product) => product,
        None => {
            return ().into_response();
        }
    };

    // update product
    services::product::update(&product.product.id, &input, &db).await;

    // detach variants
    product.product.detach_variants(&db).await;

    // attach variants
    product.product.attach_variants(&input.variants, &db).await;

    ().into_response()
}

pub async fn destroy(Path(id): Path<Uuid>, State(db): State<DB>) -> impl IntoResponse {
    services::product::delete(&id, &db).await;
}

use crate::{
    services,
    utils::db::DB,
    validations::category::{StoreCategorySchema, UpdateCategorySchema},
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
    let categories = match services::category::all(&db).await {
        Ok(categories) => categories,
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    Json(categories).into_response()
}

pub async fn show(Path(id): Path<Uuid>, State(db): State<DB>) -> impl IntoResponse {
    let category = match services::category::find(&id, &db).await {
        Ok(Some(category)) => category,
        Ok(None) => {
            return (StatusCode::NOT_FOUND).into_response();
        }
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    Json(category).into_response()
}

pub async fn store(
    State(db): State<DB>,
    Json(input): Json<StoreCategorySchema>,
) -> impl IntoResponse {
    if let Err(err) = services::category::insert(&input, &db).await {
        error!("{err}");
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    }

    (StatusCode::CREATED).into_response()
}

pub async fn update(
    Path(id): Path<Uuid>,
    State(db): State<DB>,
    Json(input): Json<UpdateCategorySchema>,
) -> impl IntoResponse {
    if let Err(err) = services::category::update(&id, &input, &db).await {
        error!("{err}");
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    }

    ().into_response()
}

pub async fn destroy(Path(id): Path<Uuid>, State(db): State<DB>) -> impl IntoResponse {
    if let Err(err) = services::category::destroy(&id, &db).await {
        error!("{err}");
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    }

    ().into_response()
}

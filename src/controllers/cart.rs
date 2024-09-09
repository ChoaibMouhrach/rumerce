use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use log::error;
use uuid::Uuid;

use crate::{services, utils::db::DB, validations::cart::StoreCartSchema};

pub async fn index(State(db): State<DB>) -> impl IntoResponse {
    let mut connection = match db.acquire().await {
        Ok(connection) => connection,
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    let carts = match services::cart::all(&mut connection).await {
        Ok(carts) => carts,
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    (Json(carts)).into_response()
}

pub async fn show(Path(id): Path<Uuid>, State(db): State<DB>) -> impl IntoResponse {
    let mut connection = match db.acquire().await {
        Ok(connection) => connection,
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    match services::cart::find(&id, &mut connection).await {
        Ok(Some(cart)) => Json(cart).into_response(),
        Ok(None) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
        Err(err) => {
            error!("{err}");
            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}

pub async fn store(State(db): State<DB>, Json(input): Json<StoreCartSchema>) -> impl IntoResponse {
    let mut connection = match db.acquire().await {
        Ok(connection) => connection,
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    if let Err(err) = services::cart::insert(&input, &mut connection).await {
        error!("{err}");
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    }

    (StatusCode::CREATED).into_response()
}

pub async fn update(
    Path(id): Path<Uuid>,
    State(db): State<DB>,
    Json(input): Json<StoreCartSchema>,
) -> impl IntoResponse {
    let mut connection = match db.acquire().await {
        Ok(connection) => connection,
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    if let Err(err) = services::cart::update(&id, &input, &mut connection).await {
        error!("{err}");
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    }

    ().into_response()
}

pub async fn destroy(Path(id): Path<Uuid>, State(db): State<DB>) -> impl IntoResponse {
    let mut connection = match db.acquire().await {
        Ok(connection) => connection,
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    if let Err(err) = services::cart::destroy(&id, &mut connection).await {
        error!("{err}");
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    }

    ().into_response()
}

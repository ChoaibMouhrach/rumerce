use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use log::error;
use sqlx::Acquire;
use uuid::Uuid;

use crate::{services, validations::product::StoreProductSchema, AppState};

pub async fn index(State(state): State<AppState>) -> impl IntoResponse {
    let mut connection = match state.db.acquire().await {
        Ok(connection) => connection,
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    let products = match services::product::all(&mut connection).await {
        Ok(products) => products,
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    Json(products).into_response()
}

pub async fn show(Path(id): Path<Uuid>, State(state): State<AppState>) -> impl IntoResponse {
    let mut connection = match state.db.acquire().await {
        Ok(connection) => connection,
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    let product = match services::product::find(&id, &mut connection).await {
        Ok(Some(product)) => product,
        Ok(None) => {
            return (StatusCode::NOT_FOUND).into_response();
        }
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    Json(product).into_response()
}

pub async fn store(
    State(state): State<AppState>,
    Json(input): Json<StoreProductSchema>,
) -> impl IntoResponse {
    let mut connection = match state.db.acquire().await {
        Ok(connection) => connection,
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    let mut tx = match connection.begin().await {
        Ok(tx) => tx,
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    let product = match services::product::insert(&input, &mut tx).await {
        Ok(product) => product,
        Err(err) => {
            error!("{err}");

            if let Err(err) = tx.rollback().await {
                error!("{err}");
            }

            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    // attach variants
    if let Err(err) = product.attach_variants(&input.variants, &mut tx).await {
        error!("{err}");

        if let Err(err) = tx.rollback().await {
            error!("{err}");
        }

        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    }

    // attach variants
    if let Err(err) = product.attach_images(&input.images, &mut tx).await {
        error!("{err}");

        if let Err(err) = tx.rollback().await {
            error!("{err}");
        }

        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    }

    if let Err(err) = tx.commit().await {
        error!("{err}");
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    }

    ().into_response()
}

pub async fn update(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    Json(input): Json<StoreProductSchema>,
) -> impl IntoResponse {
    let mut connection = match state.db.acquire().await {
        Ok(connection) => connection,
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    let product = match services::product::find(&id, &mut connection).await {
        Ok(Some(product)) => product,
        Ok(None) => {
            return (StatusCode::NOT_FOUND).into_response();
        }
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    let mut tx = match connection.begin().await {
        Ok(tx) => tx,
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    // update product
    if let Err(err) = services::product::update(&product.product.id, &input, &mut tx).await {
        error!("{err}");
        if let Err(err) = tx.rollback().await {
            error!("{err}");
        }
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    };

    // detach variants
    if let Err(err) = product.product.detach_variants(&mut tx).await {
        error!("{err}");
        if let Err(err) = tx.rollback().await {
            error!("{err}");
        }
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    };

    // attach variants
    if let Err(err) = product
        .product
        .attach_variants(&input.variants, &mut tx)
        .await
    {
        error!("{err}");
        if let Err(err) = tx.rollback().await {
            error!("{err}");
        }
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    }

    if let Err(err) = tx.commit().await {
        error!("{err}");
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    }

    (StatusCode::NO_CONTENT).into_response()
}

pub async fn destroy(Path(id): Path<Uuid>, State(state): State<AppState>) -> impl IntoResponse {
    let mut connection = match state.db.acquire().await {
        Ok(connection) => connection,
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    if let Err(err) = services::product::delete(&id, &mut connection).await {
        error!("{err}");
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    }

    (StatusCode::NO_CONTENT).into_response()
}

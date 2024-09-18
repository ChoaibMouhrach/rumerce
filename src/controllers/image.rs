use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use log::error;
use uuid::Uuid;

use crate::{
    services::{self},
    validations::image::StoreImageSchema,
    AppState,
};

pub async fn upload(State(state): State<AppState>, mut multipart: Multipart) -> impl IntoResponse {
    let file = match multipart.next_field().await {
        Ok(Some(file)) => file,
        Ok(None) => {
            return (StatusCode::BAD_REQUEST, Json("File not found")).into_response();
        }
        Err(_) => {
            return (StatusCode::BAD_REQUEST, Json("Invalid file")).into_response();
        }
    };

    let content_type = match file.content_type() {
        Some(content_type) => content_type,
        None => {
            return (StatusCode::BAD_REQUEST, Json("Invalid content type")).into_response();
        }
    };

    if content_type != "image/jpeg" || content_type != "image/png" {
        return (
            StatusCode::BAD_REQUEST,
            Json("Only image file type is allowed"),
        )
            .into_response();
    }

    let file_name = match file.name() {
        Some(file_name) => file_name.to_string(),
        None => {
            return (StatusCode::BAD_REQUEST, Json("Couldn't find file name")).into_response();
        }
    };

    let bytes = match file.bytes().await {
        Ok(bytes) => bytes,
        Err(_) => {
            return (StatusCode::BAD_REQUEST, Json("Couldn't decode")).into_response();
        }
    };

    let mut connection = match state.db.acquire().await {
        Ok(connection) => connection,
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    let image = services::image::upload(
        &StoreImageSchema {
            name: file_name.to_string(),
            src: format!("{}-{}", Uuid::new_v4(), file_name),
        },
        &bytes,
        &mut connection,
    )
    .await;

    if let Err(err) = image {
        error!("{err}");
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    }

    (StatusCode::CREATED).into_response()
}

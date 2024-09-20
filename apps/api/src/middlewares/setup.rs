use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use log::error;

use crate::{services, utils::constants::SETTINGS, AppState};

pub async fn middleware(State(state): State<AppState>, request: Request, next: Next) -> Response {
    if *request.uri() == "/setup" {
        return next.run(request).await;
    }

    let mut connection = match state.db.acquire().await {
        Ok(connection) => connection,
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    let setup = services::settings::find_by_key(SETTINGS.setup, &mut connection).await;

    if let Ok(None) = setup {
        return (StatusCode::FAILED_DEPENDENCY).into_response();
    }

    if let Err(err) = setup {
        error!("{err}");
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    }

    next.run(request).await
}

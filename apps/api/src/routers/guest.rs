use crate::{
    controllers::{
        self,
        auth::{auth, sign_in},
    },
    services, AppState,
};

use axum::{
    extract::{Request, State},
    http::{header, HeaderMap, StatusCode},
    middleware::{from_fn_with_state, Next},
    response::{IntoResponse, Response},
    routing::{delete, get, patch, post},
    Router,
};
use biscotti::{Processor, ProcessorConfig, RequestCookies};
use log::error;
use uuid::Uuid;

async fn auth_middleware(
    State(state): State<AppState>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Response {
    let mut connection = match state.db.acquire().await {
        Ok(connection) => connection,
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    let session = headers
        .get(header::COOKIE)
        .and_then(|cookies| cookies.to_str().ok())
        .and_then(|cookies| {
            let processor: Processor = ProcessorConfig::default().into();
            RequestCookies::parse_header(cookies, &processor).ok()
        })
        .and_then(|cookies| cookies.get("session"))
        .and_then(|cookie| Some(cookie.value().to_string()))
        .and_then(|session| Uuid::parse_str(&session).ok());

    let session = match session {
        Some(session) => {
            match services::sessions::find_by_session(&session, &mut connection).await {
                Ok(session) => session,
                Err(err) => {
                    error!("{err}");
                    return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
                }
            }
        }
        None => None,
    };

    request.extensions_mut().insert(session);
    next.run(request).await
}

pub fn init(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/sign-in", post(sign_in))
        .route("/auth", get(auth))
        .route("/carts", get(controllers::cart::list_items))
        .route("/carts", post(controllers::cart::add_item))
        .route("/carts/:item_id", patch(controllers::cart::update_item))
        .route("/carts/:item_id", delete(controllers::cart::delete_item))
        .route_layer(from_fn_with_state(state, auth_middleware))
}

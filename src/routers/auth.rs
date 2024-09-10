use crate::{
    controllers::{auth, category, product, role, unit, user, warehouse},
    services, AppState,
};

use axum::{
    extract::{Request, State},
    http::{header, HeaderMap, StatusCode},
    middleware::{from_fn_with_state, Next},
    response::{IntoResponse, Response},
    routing::{delete, get, patch, post},
    Json, Router,
};
use cookie::Cookie;
use log::error;
use uuid::Uuid;

async fn auth_middleware(
    State(state): State<AppState>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Response {
    let cookies = match headers.get(header::COOKIE) {
        Some(cookies) => cookies,
        None => {
            return (StatusCode::BAD_REQUEST, Json("Missing session")).into_response();
        }
    };

    let cookies = match cookies.to_str() {
        Ok(cookies) => cookies,
        Err(_) => {
            return (StatusCode::BAD_REQUEST, Json("Invalid cookies")).into_response();
        }
    };

    let cookies: Vec<&str> = cookies.split("; ").collect();
    let mut target_cookie: Option<Cookie> = None;

    for cookie in cookies {
        let cookie = match Cookie::parse(cookie) {
            Ok(cookie) => cookie,
            Err(_) => {
                return (StatusCode::BAD_REQUEST, "Invalid cookies").into_response();
            }
        };

        if cookie.name() == "session" {
            target_cookie = Some(cookie);
            break;
        }
    }

    let session_cookie = match target_cookie {
        Some(target_cookie) => target_cookie,
        None => {
            return (StatusCode::BAD_REQUEST, Json("Session not found")).into_response();
        }
    };

    let session = match Uuid::parse_str(session_cookie.value()) {
        Ok(session) => session,
        Err(_) => {
            return (StatusCode::BAD_REQUEST, Json("Invalid session")).into_response();
        }
    };

    let mut connection = match state.db.acquire().await {
        Ok(connection) => connection,
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    let session = match services::sessions::find_by_session(&session, &mut connection).await {
        Ok(Some(session)) => session,
        Ok(None) => {
            return (StatusCode::UNAUTHORIZED).into_response();
        }
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    request.extensions_mut().insert(session);
    next.run(request).await
}

pub fn init(state: AppState) -> Router<AppState> {
    let role_router = Router::new()
        .route("/roles", get(role::index))
        .route("/roles/:id", get(role::show))
        .route("/roles", post(role::store))
        .route("/roles/:id", patch(role::update))
        .route("/roles/:id", delete(role::destroy));

    let user_router = Router::new()
        .route("/users", get(user::index))
        .route("/users/:id", get(user::show))
        .route("/users", post(user::store))
        .route("/users/:id", patch(user::update))
        .route("/users/:id", delete(user::destroy));

    let category_router = Router::new()
        .route("/categories", get(category::index))
        .route("/categories/:id", get(category::show))
        .route("/categories", post(category::store))
        .route("/categories/:id", patch(category::update))
        .route("/categories/:id", delete(category::destroy));

    let unit_router = Router::new()
        .route("/units", get(unit::index))
        .route("/units/:id", get(unit::show))
        .route("/units", post(unit::store))
        .route("/units/:id", patch(unit::update))
        .route("/units/:id", delete(unit::destroy));

    let warehouse_router = Router::new()
        .route("/warehouses", get(warehouse::index))
        .route("/warehouses/:id", get(warehouse::show))
        .route("/warehouses", post(warehouse::store))
        .route("/warehouses/:id", patch(warehouse::update))
        .route("/warehouses/:id", delete(warehouse::destroy));

    let product_router = Router::new()
        .route("/products", get(product::index))
        .route("/products/:id", get(product::show))
        .route("/products", post(product::store))
        .route("/products/:id", patch(product::update))
        .route("/products/:id", delete(product::destroy));

    let auth_router = Router::new()
        .route("/sign-out", post(auth::sign_out))
        .route("/profile", get(auth::profile));

    Router::new()
        .merge(role_router)
        .merge(user_router)
        .merge(category_router)
        .merge(unit_router)
        .merge(warehouse_router)
        .merge(product_router)
        .merge(auth_router)
        .route_layer(from_fn_with_state(state, auth_middleware))
}

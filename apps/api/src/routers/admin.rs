use axum::{
    middleware::{from_fn, from_fn_with_state},
    routing::{delete, get, patch, post},
    Router,
};
use tower_http::limit::RequestBodyLimitLayer;

use crate::{
    controllers::{category, image, product, role, unit, user, warehouse},
    middlewares::{admin::admin_middleware, auth::auth_middleware},
    AppState,
};

pub fn init(state: AppState) -> Router<AppState> {
    let role_router = Router::new()
        .route("/roles", get(role::index))
        .route("/roles/:id", get(role::show));

    let user_router = Router::new()
        .route("/users", get(user::index))
        .route("/users/:id", get(user::show))
        .route("/users", post(user::store))
        .route("/users/:id", patch(user::update))
        .route("/users/:id", delete(user::destroy));

    let category_router = Router::new()
        .route("/categories", post(category::store))
        .route("/categories/:id", patch(category::update))
        .route("/categories/:id", delete(category::destroy));

    let unit_router = Router::new()
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
        .route("/products", post(product::store))
        .route("/products/:id", patch(product::update))
        .route("/products/:id", delete(product::destroy));

    let image_router = Router::new()
        .route("/images", post(image::upload))
        .layer(RequestBodyLimitLayer::new(2 * 1024 * 1024));

    Router::new()
        .merge(role_router)
        .merge(user_router)
        .merge(category_router)
        .merge(unit_router)
        .merge(warehouse_router)
        .merge(product_router)
        .merge(image_router)
        .route_layer(from_fn(admin_middleware))
        .route_layer(from_fn_with_state(state, auth_middleware))
}

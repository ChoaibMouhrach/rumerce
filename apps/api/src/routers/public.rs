use axum::{
    middleware::from_fn_with_state,
    routing::{delete, get, patch, post},
    Router,
};
use tower_http::services::ServeDir;

use crate::{
    controllers::{auth, cart, category, product, unit},
    middlewares::optional_auth,
    AppState,
};

pub fn init(state: AppState) -> Router<AppState> {
    let category_router = Router::new()
        .route("/categories", get(category::index))
        .route("/categories/:id", get(category::show));

    let unit_router = Router::new()
        .route("/units", get(unit::index))
        .route("/units/:id", get(unit::show));

    let product_router = Router::new()
        .route("/products", get(product::index))
        .route("/products/:id", get(product::show));

    let cart_router = Router::new()
        .route("/carts", get(cart::list_items))
        .route("/carts", post(cart::add_item))
        .route("/carts/:item_id", patch(cart::update_item))
        .route("/carts/:item_id", delete(cart::delete_item));

    let auth_router = Router::new()
        .route("/sign-in", post(auth::sign_in))
        .route("/auth", get(auth::auth));

    let public_router = Router::new().nest_service("/public", ServeDir::new("public"));

    Router::new()
        .merge(category_router)
        .merge(unit_router)
        .merge(product_router)
        .merge(public_router)
        .merge(cart_router)
        .merge(auth_router)
        .route_layer(from_fn_with_state(state, optional_auth::middleware))
}

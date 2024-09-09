use crate::{
    controllers::{cart, category, product, role, unit, user, warehouse},
    utils::db::DB,
};
use axum::{
    routing::{delete, get, patch, post},
    Router,
};

pub fn init() -> Router<DB> {
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

    let cart_router = Router::new()
        .route("/carts", get(cart::index))
        .route("/carts/:id", get(cart::show))
        .route("/carts", post(cart::store))
        .route("/carts/:id", patch(cart::update))
        .route("/carts/:id", delete(cart::destroy));

    Router::new()
        .merge(role_router)
        .merge(user_router)
        .merge(category_router)
        .merge(unit_router)
        .merge(warehouse_router)
        .merge(product_router)
        .merge(cart_router)
}

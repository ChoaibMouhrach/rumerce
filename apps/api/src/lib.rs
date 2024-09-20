use std::sync::Arc;

use axum::{
    http::{header::CONTENT_TYPE, HeaderValue, Method},
    Router,
};
use routers::{admin, auth, public};
use tower_http::cors::CorsLayer;
use utils::{db::DB, env::Env, mailer::Mail};

pub mod controllers;
pub mod middlewares;
pub mod models;
pub mod routers;
pub mod services;
pub mod utils;
pub mod validations;

#[derive(Clone)]
pub struct State {
    pub db: DB,
    pub env: Env,
    pub mailer: Box<dyn Mail>,
}

pub type AppState = Arc<State>;

pub fn create_app(state: State) -> Router {
    let state = Arc::new(state);

    let auth_router = auth::init(state.clone());
    let admin_router = admin::init(state.clone());
    let public_router = public::init(state.clone());

    Router::new()
        .merge(auth_router)
        .merge(admin_router)
        .merge(public_router)
        .with_state(state.clone())
        .layer(
            CorsLayer::new()
                .allow_origin(state.env.client_url.parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
                .allow_headers([CONTENT_TYPE])
                .allow_credentials(true),
        )
}

use std::sync::Arc;

use axum::{
    http::{header::CONTENT_TYPE, HeaderValue, Method},
    Router,
};
use resend_rs::Resend;
use routers::{auth, guest};
use tower_http::cors::CorsLayer;
use utils::{
    db::{self, DB},
    env::{self, Env},
};

pub mod controllers;
pub mod models;
pub mod routers;
pub mod services;
pub mod utils;
pub mod validations;

#[derive(Clone)]
pub struct State {
    pub db: DB,
    pub env: Env,
    pub resend: Resend,
}

pub type AppState = Arc<State>;

pub async fn run() {
    tracing_subscriber::fmt::init();

    let env = env::init();
    let db = db::init(&env.database_url).await;
    let resend = Resend::new(&env.resend_token);

    let state = Arc::new(State { env, db, resend });

    let guest_router = guest::init(state.clone());
    let auth_router = auth::init(state.clone());
    let app = Router::new()
        .merge(guest_router)
        .merge(auth_router)
        //TODO: Investigate more
        .with_state(state.clone())
        .layer(
            CorsLayer::new()
                .allow_origin(state.env.client_url.parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
                .allow_headers([CONTENT_TYPE])
                .allow_credentials(true),
        );

    println!("Server running on {:#?}", &state.env.app_url);
    let listener = tokio::net::TcpListener::bind(&state.env.app_url)
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

use axum::{response::IntoResponse, Router};
use routers::guest;
use utils::{db, env};

pub mod controllers;
pub mod models;
pub mod routers;
pub mod services;
pub mod utils;
pub mod validations;

pub async fn handle() -> impl IntoResponse {
    "Hello world"
}

pub async fn run() {
    tracing_subscriber::fmt::init();

    let env = env::init();
    let db = db::init(&env.database_url).await;

    let guest_router = guest::init();
    let app = Router::new().merge(guest_router).with_state(db);

    println!("Server running on {:#?}", env.app_url);
    let listener = tokio::net::TcpListener::bind(env.app_url).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

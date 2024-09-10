use crate::{
    controllers::auth::{auth, sign_in},
    AppState,
};

use axum::{
    routing::{get, post},
    Router,
};

pub fn init() -> Router<AppState> {
    Router::new()
        .route("/sign-in", post(sign_in))
        .route("/auth", get(auth))
}

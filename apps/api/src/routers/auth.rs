use crate::{controllers::auth, middlewares::auth::auth_middleware, AppState};

use axum::{
    middleware::from_fn_with_state,
    routing::{get, post},
    Router,
};

pub fn init(state: AppState) -> Router<AppState> {
    let auth_router = Router::new()
        .route("/sign-out", post(auth::sign_out))
        .route("/profile", get(auth::profile));

    Router::new()
        .merge(auth_router)
        .route_layer(from_fn_with_state(state, auth_middleware))
}

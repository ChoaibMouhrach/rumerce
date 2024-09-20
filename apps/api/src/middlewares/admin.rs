use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Extension,
};

use crate::{models::session::PopulatedSession, utils::constants::ROLES};

pub async fn admin_middleware(
    Extension(session): Extension<PopulatedSession>,
    request: Request,
    next: Next,
) -> Response {
    if session.role.name != ROLES.admin {
        return (StatusCode::FORBIDDEN).into_response();
    }

    next.run(request).await
}

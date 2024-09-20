use axum::{
    extract::{Request, State},
    http::{header, HeaderMap, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use cookie::Cookie;
use log::error;
use uuid::Uuid;

use crate::{services, AppState};

pub async fn auth_middleware(
    State(state): State<AppState>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Response {
    let cookies = match headers.get(header::COOKIE) {
        Some(cookies) => cookies,
        None => {
            return (StatusCode::UNAUTHORIZED, Json("Missing session")).into_response();
        }
    };

    let cookies = match cookies.to_str() {
        Ok(cookies) => cookies,
        Err(_) => {
            return (StatusCode::UNAUTHORIZED, Json("Invalid cookies")).into_response();
        }
    };

    let cookies: Vec<&str> = cookies.split("; ").collect();
    let mut target_cookie: Option<Cookie> = None;

    for cookie in cookies {
        let cookie = match Cookie::parse(cookie) {
            Ok(cookie) => cookie,
            Err(_) => {
                return (StatusCode::UNAUTHORIZED, "Invalid cookies").into_response();
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
            return (StatusCode::UNAUTHORIZED, Json("Session not found")).into_response();
        }
    };

    let session = match Uuid::parse_str(session_cookie.value()) {
        Ok(session) => session,
        Err(_) => {
            return (StatusCode::UNAUTHORIZED, Json("Invalid session")).into_response();
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

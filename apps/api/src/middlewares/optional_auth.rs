use axum::{
    extract::{Request, State},
    http::{header, HeaderMap, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use cookie::Cookie;
use log::error;
use uuid::Uuid;

use crate::{models::session::PopulatedSession, services, AppState};

pub async fn middleware(
    State(state): State<AppState>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Response {
    let cookies = match headers.get(header::COOKIE) {
        Some(cookies) => cookies,
        None => {
            let session: Option<PopulatedSession> = None;
            request.extensions_mut().insert(session);
            return next.run(request).await;
        }
    };

    let cookies = match cookies.to_str() {
        Ok(cookies) => cookies,
        Err(_) => {
            let session: Option<PopulatedSession> = None;
            request.extensions_mut().insert(session);
            return next.run(request).await;
        }
    };

    let cookies: Vec<&str> = cookies.split("; ").collect();
    let mut target_cookie: Option<Cookie> = None;

    for cookie in cookies {
        let cookie = match Cookie::parse(cookie) {
            Ok(cookie) => cookie,
            Err(_) => {
                let session: Option<PopulatedSession> = None;
                request.extensions_mut().insert(session);
                return next.run(request).await;
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
            let session: Option<PopulatedSession> = None;
            request.extensions_mut().insert(session);
            return next.run(request).await;
        }
    };

    let session = match Uuid::parse_str(session_cookie.value()) {
        Ok(session) => session,
        Err(_) => {
            let session: Option<PopulatedSession> = None;
            request.extensions_mut().insert(session);
            return next.run(request).await;
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
        Ok(session) => session,
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    request.extensions_mut().insert(session);

    next.run(request).await
}

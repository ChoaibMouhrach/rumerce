use std::collections::HashMap;

use axum::{body::Body, http::Request};
use common::init;
use http_body_util::BodyExt;
use rumerce::{services, utils::constants::ROLES};
use testcontainers::{runners::AsyncRunner, ImageExt};
use testcontainers_modules::postgres::Postgres;
use tower::ServiceExt;
use uuid::Uuid;

mod common;

#[tokio::test]
pub async fn sign_in_success() {
    let container = Postgres::default()
        .with_tag("latest")
        .start()
        .await
        .unwrap();

    let mut config = init(&container).await;

    let email = "example@example.com";

    let mut body = HashMap::new();
    body.insert("email", email);

    let response = config
        .app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/sign-in")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 200);

    let user = services::user::find_by_email(&email, &mut config.connection)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(user.user.email, email);
    assert_eq!(user.role.name, ROLES.member);

    services::magic_tokens::find_by_user(&user.user.id, &mut config.connection)
        .await
        .unwrap()
        .unwrap();
}

#[tokio::test]
pub async fn sign_in_no_body() {
    let container = Postgres::default()
        .with_tag("latest")
        .start()
        .await
        .unwrap();

    let mut config = init(&container).await;

    let response = config
        .app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/sign-in")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 400);

    let users = services::user::all(&mut config.connection).await.unwrap();
    assert_eq!(users.len(), 0);

    let magic_tokens = services::magic_tokens::all(&mut config.connection)
        .await
        .unwrap();
    assert_eq!(magic_tokens.len(), 0);
}

#[tokio::test]
pub async fn sign_in_invalid_email() {
    let container = Postgres::default()
        .with_tag("latest")
        .start()
        .await
        .unwrap();

    let mut config = init(&container).await;

    let mut body = HashMap::new();
    body.insert("email", "example");

    let response = config
        .app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/sign-in")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 400);

    let users = services::user::all(&mut config.connection).await.unwrap();
    assert_eq!(users.len(), 0);

    let magic_tokens = services::magic_tokens::all(&mut config.connection)
        .await
        .unwrap();
    assert_eq!(magic_tokens.len(), 0);
}

#[tokio::test]
pub async fn sign_in_no_email() {
    let container = Postgres::default()
        .with_tag("latest")
        .start()
        .await
        .unwrap();

    let mut config = init(&container).await;

    let mut body = HashMap::new();
    body.insert("email", "");

    let response = config
        .app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/sign-in")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 400);

    let users = services::user::all(&mut config.connection).await.unwrap();
    assert_eq!(users.len(), 0);

    let magic_tokens = services::magic_tokens::all(&mut config.connection)
        .await
        .unwrap();
    assert_eq!(magic_tokens.len(), 0);
}

// AUTH
#[tokio::test]
pub async fn auth_success() {
    let container = Postgres::default()
        .with_tag("latest")
        .start()
        .await
        .unwrap();

    let mut config = init(&container).await;

    let email = "example@example.com";

    let mut body = HashMap::new();
    body.insert("email", email);

    let response = config
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/sign-in")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 200);

    let user = services::user::find_by_email(&email, &mut config.connection)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(user.user.email, email);
    assert_eq!(user.role.name, ROLES.member);

    let token = services::magic_tokens::find_by_user(&user.user.id, &mut config.connection)
        .await
        .unwrap()
        .unwrap();

    let response = config
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/auth?token={}", token.token.token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 200);

    // check cookie
    assert_eq!(response.headers().get("set-cookie").is_some(), true);
    let value = response
        .headers()
        .get("set-cookie")
        .unwrap()
        .to_str()
        .unwrap();

    assert_eq!(value.contains("session"), true);

    let tokens = services::magic_tokens::all(&mut config.connection)
        .await
        .unwrap();
    assert_eq!(tokens.len(), 0);

    let sessions = services::sessions::all(&mut config.connection)
        .await
        .unwrap();
    assert_ne!(sessions.len(), 0);
}

#[tokio::test]
pub async fn auth_no_token() {
    let container = Postgres::default()
        .with_tag("latest")
        .start()
        .await
        .unwrap();

    let mut config = init(&container).await;

    let response = config
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/auth?token=")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 400);

    let sessions = services::sessions::all(&mut config.connection)
        .await
        .unwrap();
    assert_eq!(sessions.len(), 0);
}

#[tokio::test]
pub async fn auth_no_token_param() {
    let container = Postgres::default()
        .with_tag("latest")
        .start()
        .await
        .unwrap();

    let mut config = init(&container).await;

    let response = config
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/auth")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 400);

    let sessions = services::sessions::all(&mut config.connection)
        .await
        .unwrap();
    assert_eq!(sessions.len(), 0);
}

#[tokio::test]
pub async fn auth_no_token_found() {
    let container = Postgres::default()
        .with_tag("latest")
        .start()
        .await
        .unwrap();

    let mut config = init(&container).await;

    let response = config
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/auth?token={}", Uuid::new_v4().to_string()))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 400);

    let sessions = services::sessions::all(&mut config.connection)
        .await
        .unwrap();
    assert_eq!(sessions.len(), 0);
}

// SIGN OUT
#[tokio::test]
pub async fn sign_out_success() {
    let container = Postgres::default()
        .with_tag("latest")
        .start()
        .await
        .unwrap();

    let mut config = init(&container).await;

    let email = "example@example.com";

    let mut body = HashMap::new();
    body.insert("email", email);

    let response = config
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/sign-in")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 200);

    let user = services::user::find_by_email(&email, &mut config.connection)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(user.user.email, email);
    assert_eq!(user.role.name, ROLES.member);

    let token = services::magic_tokens::find_by_user(&user.user.id, &mut config.connection)
        .await
        .unwrap()
        .unwrap();

    let response = config
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/auth?token={}", token.token.token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 200);

    // check cookie
    assert_eq!(response.headers().get("set-cookie").is_some(), true);
    let value = response
        .headers()
        .get("set-cookie")
        .unwrap()
        .to_str()
        .unwrap();

    assert_eq!(value.contains("session"), true);

    let tokens = services::magic_tokens::all(&mut config.connection)
        .await
        .unwrap();
    assert_eq!(tokens.len(), 0);

    let session = services::sessions::find_by_user_id(&user.user.id, &mut config.connection)
        .await
        .unwrap()
        .unwrap();

    let response = config
        .app
        .clone()
        .oneshot(
            Request::builder()
                .header("COOKIE", &format!("session={}", session.session.session))
                .method("POST")
                .uri("/sign-out")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 200);

    let session = services::sessions::find_by_user_id(&user.user.id, &mut config.connection)
        .await
        .unwrap();

    assert_eq!(session.is_some(), false);
}

#[tokio::test]
pub async fn sign_out_no_session() {
    let container = Postgres::default()
        .with_tag("latest")
        .start()
        .await
        .unwrap();

    let config = init(&container).await;

    let response = config
        .app
        .clone()
        .oneshot(
            Request::builder()
                .header("COOKIE", "session=")
                .method("POST")
                .uri("/sign-out")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 401);
}

#[tokio::test]
pub async fn profile_success() {
    let container = Postgres::default()
        .with_tag("latest")
        .start()
        .await
        .unwrap();

    let mut config = init(&container).await;

    let email = "example@example.com";

    let mut body = HashMap::new();
    body.insert("email", email);

    let response = config
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/sign-in")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 200);

    let user = services::user::find_by_email(&email, &mut config.connection)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(user.user.email, email);
    assert_eq!(user.role.name, ROLES.member);

    let token = services::magic_tokens::find_by_user(&user.user.id, &mut config.connection)
        .await
        .unwrap()
        .unwrap();

    let response = config
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/auth?token={}", token.token.token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 200);

    // check cookie
    assert_eq!(response.headers().get("set-cookie").is_some(), true);
    let value = response
        .headers()
        .get("set-cookie")
        .unwrap()
        .to_str()
        .unwrap();

    assert_eq!(value.contains("session"), true);

    let tokens = services::magic_tokens::all(&mut config.connection)
        .await
        .unwrap();
    assert_eq!(tokens.len(), 0);

    let session = services::sessions::find_by_user_id(&user.user.id, &mut config.connection)
        .await
        .unwrap()
        .unwrap();

    let response = config
        .app
        .clone()
        .oneshot(
            Request::builder()
                .header("COOKIE", &format!("session={}", session.session.session))
                .method("GET")
                .uri("/profile")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let value: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(value["user"]["email"], email);
}

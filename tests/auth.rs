use std::collections::HashMap;

use axum::{body::Body, http::Request};
use common::init;
use rumerce::{services, utils::constants::ROLES};
use testcontainers::{runners::AsyncRunner, ImageExt};
use testcontainers_modules::postgres::Postgres;
use tower::ServiceExt;

mod common;

#[tokio::test]
async fn sign_in_works() {
    let container = Postgres::default()
        .with_tag("latest")
        .start()
        .await
        .unwrap();

    let (app, mut connection) = init(&container).await;

    let email = "example@example.com";

    let mut body = HashMap::new();
    body.insert("email", email);

    let response = app
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

    let user = services::user::find_by_email(email, &mut connection)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(user.user.email, email);
    assert_eq!(user.role.name, ROLES.member);
}

#[tokio::test]
async fn auth_works() {
    let container = Postgres::default()
        .with_tag("latest")
        .start()
        .await
        .unwrap();

    let (app, mut connection) = init(&container).await;

    let email = "example@example.com";

    let mut body = HashMap::new();
    body.insert("email", email);

    let response = app
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

    let user = services::user::find_by_email(email, &mut connection)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(user.user.email, email);
    assert_eq!(user.role.name, ROLES.member);

    let token = services::magic_tokens::find_by_user(&user.user.id, &mut connection)
        .await
        .unwrap()
        .unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(&format!("/auth?token={}", token.token.token))
                .header("Content-Type", "application/json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 200);

    let token_found = services::magic_tokens::find_by_token(&token.token.id, &mut connection)
        .await
        .unwrap()
        .is_some();

    assert_eq!(token_found, false);

    let session_found = services::sessions::find_by_user_id(&token.user.id, &mut connection)
        .await
        .unwrap()
        .is_some();

    assert_eq!(session_found, true);
}

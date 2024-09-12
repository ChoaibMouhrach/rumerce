use std::collections::HashMap;

use axum::{body::Body, http::Request};
use common::init;
use rumerce::{services, utils::constants::ROLES};
use testcontainers::{runners::AsyncRunner, ImageExt};
use testcontainers_modules::postgres::Postgres;
use tower::ServiceExt;

mod common;

#[tokio::test]
async fn run() {
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

use axum::{body::Body, http::Request};
use testcontainers::{runners::AsyncRunner, ImageExt};
use testcontainers_modules::postgres::Postgres;
use tower::ServiceExt;
use uuid::Uuid;

use common::init;

mod common;

async fn test(method: &str, path: &str) {
    let container = Postgres::default()
        .with_tag("latest")
        .start()
        .await
        .unwrap();

    let config = init(&container).await;

    let response = config
        .app
        .oneshot(
            Request::builder()
                .method(method)
                .uri(path)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 401);
}

#[tokio::test]
pub async fn unauth() {
    let protected_routes = [
        // AUTH
        ("GET", "/profile"),
        // CATEGORIES
        ("GET", "/categories"),
        (
            "GET",
            &format!("/categories/{}", Uuid::new_v4().to_string()),
        ),
        ("POST", "/categories"),
        (
            "PATCH",
            &format!("/categories/{}", Uuid::new_v4().to_string()),
        ),
        (
            "DELETE",
            &format!("/categories/{}", Uuid::new_v4().to_string()),
        ),
        // UNITS
        ("GET", "/units"),
        ("GET", &format!("/units/{}", Uuid::new_v4().to_string())),
        ("POST", "/units"),
        ("PATCH", &format!("/units/{}", Uuid::new_v4().to_string())),
        ("DELETE", &format!("/units/{}", Uuid::new_v4().to_string())),
    ];

    let tests = protected_routes
        .into_iter()
        .map(|(method, path)| test(method, path))
        .collect::<Vec<_>>();

    futures::future::join_all(tests).await;
}

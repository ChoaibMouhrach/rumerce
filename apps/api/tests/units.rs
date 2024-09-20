use std::collections::HashMap;

use axum::{body::Body, http::Request};
use common::{auth, init};

use http_body_util::BodyExt;
use rumerce::{services, validations::unit::StoreUnitSchema};
use testcontainers::{runners::AsyncRunner, ImageExt};
use testcontainers_modules::postgres::Postgres;
use tower::ServiceExt;
use uuid::Uuid;

mod common;

#[tokio::test]
pub async fn get_units_success() {
    let container = Postgres::default()
        .with_tag("latest")
        .start()
        .await
        .unwrap();

    let mut config = init(&container).await;
    let (_, session) = auth(&config.member, &mut config.connection).await;

    let unit = services::unit::insert(
        &StoreUnitSchema {
            name: "Unit 1".to_string(),
        },
        &mut config.connection,
    )
    .await
    .unwrap();

    let response = config
        .app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/units")
                .header("Cookie", &format!("session={}", session.session))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 200);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body: serde_json::Value = serde_json::from_slice(&bytes[..]).unwrap();
    let body = match body {
        serde_json::Value::Array(body) => body,
        _ => panic!("array expected"),
    };

    assert_eq!(body.len(), 1);
    assert_eq!(body[0]["name"], unit.name);
}

#[tokio::test]
pub async fn show_unit_success() {
    let container = Postgres::default()
        .with_tag("latest")
        .start()
        .await
        .unwrap();

    let mut config = init(&container).await;
    let (_, session) = auth(&config.member, &mut config.connection).await;

    let unit = services::unit::insert(
        &StoreUnitSchema {
            name: "Unit 1".to_string(),
        },
        &mut config.connection,
    )
    .await
    .unwrap();

    let response = config
        .app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(&format!("/units/{}", unit.id.to_string()))
                .header("Cookie", &format!("session={}", session.session))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 200);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body: serde_json::Value = serde_json::from_slice(&bytes[..]).unwrap();

    let body = match body {
        serde_json::Value::Object(body) => body,
        _ => panic!("object expected"),
    };

    assert_eq!(body["name"], unit.name);
}

#[tokio::test]
pub async fn show_unit_not_found() {
    let container = Postgres::default()
        .with_tag("latest")
        .start()
        .await
        .unwrap();

    let mut config = init(&container).await;
    let (_, session) = auth(&config.member, &mut config.connection).await;

    let response = config
        .app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(&format!("/units/{}", Uuid::new_v4().to_string()))
                .header("Cookie", &format!("session={}", session.session))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 404);
}

#[tokio::test]
pub async fn store_unit_success() {
    let container = Postgres::default()
        .with_tag("latest")
        .start()
        .await
        .unwrap();

    let mut config = init(&container).await;
    let (_, session) = auth(&config.member, &mut config.connection).await;

    let mut payload = HashMap::new();
    payload.insert("name", "Unit 1");

    let response = config
        .app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/units")
                .header("cookie", &format!("session={}", session.session))
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 201);

    services::unit::find_by_name(payload["name"], &mut config.connection)
        .await
        .unwrap()
        .unwrap();
}

#[tokio::test]
pub async fn store_unit_taken() {
    let container = Postgres::default()
        .with_tag("latest")
        .start()
        .await
        .unwrap();

    let mut config = init(&container).await;
    let (_, session) = auth(&config.member, &mut config.connection).await;

    let unit_name = "Unit 1";

    services::unit::insert(
        &StoreUnitSchema {
            name: unit_name.to_string(),
        },
        &mut config.connection,
    )
    .await
    .unwrap();

    let mut payload = HashMap::new();
    payload.insert("name", unit_name);

    let response = config
        .app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/units")
                .header("cookie", &format!("session={}", session.session))
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 403);
}

#[tokio::test]
pub async fn update_unit_success() {
    let container = Postgres::default()
        .with_tag("latest")
        .start()
        .await
        .unwrap();

    let mut config = init(&container).await;
    let (_, session) = auth(&config.member, &mut config.connection).await;

    let unit = services::unit::insert(
        &StoreUnitSchema {
            name: "Unit 1".to_string(),
        },
        &mut config.connection,
    )
    .await
    .unwrap();

    let new_unit_name = "Unit 2";

    let mut payload = HashMap::new();
    payload.insert("name", new_unit_name);

    let response = config
        .app
        .oneshot(
            Request::builder()
                .method("PATCH")
                .uri(&format!("/units/{}", unit.id.to_string()))
                .header("cookie", &format!("session={}", session.session))
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 204);

    let updated_unit = services::unit::find(&unit.id, &mut config.connection)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(updated_unit.name, new_unit_name);
}

#[tokio::test]
pub async fn update_unit_taken() {
    let container = Postgres::default()
        .with_tag("latest")
        .start()
        .await
        .unwrap();

    let mut config = init(&container).await;
    let (_, session) = auth(&config.member, &mut config.connection).await;

    let unit_name = "Unit 1";

    services::unit::insert(
        &StoreUnitSchema {
            name: unit_name.to_string(),
        },
        &mut config.connection,
    )
    .await
    .unwrap();

    let unit = services::unit::insert(
        &StoreUnitSchema {
            name: "Unit 2".to_string(),
        },
        &mut config.connection,
    )
    .await
    .unwrap();

    let mut payload = HashMap::new();
    payload.insert("name", unit_name);

    let response = config
        .app
        .oneshot(
            Request::builder()
                .method("PATCH")
                .uri(&format!("/units/{}", unit.id.to_string()))
                .header("cookie", &format!("session={}", session.session))
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 403);
}

#[tokio::test]
pub async fn delete_unit_success() {
    let container = Postgres::default()
        .with_tag("latest")
        .start()
        .await
        .unwrap();

    let mut config = init(&container).await;
    let (_, session) = auth(&config.member, &mut config.connection).await;

    let unit = services::unit::insert(
        &StoreUnitSchema {
            name: "Unit 1".to_string(),
        },
        &mut config.connection,
    )
    .await
    .unwrap();

    let response = config
        .app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(&format!("/units/{}", unit.id.to_string()))
                .header("cookie", &format!("session={}", session.session))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 204);

    let unit = services::unit::find(&unit.id, &mut config.connection)
        .await
        .unwrap();
    assert_eq!(unit.is_some(), false);
}

#[tokio::test]
pub async fn delete_unit_not_found() {
    let container = Postgres::default()
        .with_tag("latest")
        .start()
        .await
        .unwrap();

    let mut config = init(&container).await;
    let (_, session) = auth(&config.member, &mut config.connection).await;

    let response = config
        .app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(&format!("/units/{}", Uuid::new_v4().to_string()))
                .header("cookie", &format!("session={}", session.session))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 204);
}

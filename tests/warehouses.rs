use std::collections::HashMap;

use axum::{body::Body, http::Request};
use common::{auth, init};

use http_body_util::BodyExt;
use rumerce::{services, validations::warehouse::StoreWarehouseSchema};
use testcontainers::{runners::AsyncRunner, ImageExt};
use testcontainers_modules::postgres::Postgres;
use tower::ServiceExt;
use uuid::Uuid;

mod common;

#[tokio::test]
pub async fn get_warehouses_success() {
    let container = Postgres::default()
        .with_tag("latest")
        .start()
        .await
        .unwrap();

    let mut config = init(&container).await;
    let (_, session) = auth(&config.member, &mut config.connection).await;

    let warehouse = services::warehouse::insert(
        &StoreWarehouseSchema {
            name: "Warehouse 1".to_string(),
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
                .uri("/warehouses")
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
    assert_eq!(body[0]["name"], warehouse.name);
}

#[tokio::test]
pub async fn show_warehouse_success() {
    let container = Postgres::default()
        .with_tag("latest")
        .start()
        .await
        .unwrap();

    let mut config = init(&container).await;
    let (_, session) = auth(&config.member, &mut config.connection).await;

    let warehouse = services::warehouse::insert(
        &StoreWarehouseSchema {
            name: "Warehouse 1".to_string(),
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
                .uri(&format!("/warehouses/{}", warehouse.id.to_string()))
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

    assert_eq!(body["name"], warehouse.name);
}

#[tokio::test]
pub async fn show_warehouse_not_found() {
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
                .uri(&format!("/warehouses/{}", Uuid::new_v4().to_string()))
                .header("Cookie", &format!("session={}", session.session))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 404);
}

#[tokio::test]
pub async fn store_warehouse_success() {
    let container = Postgres::default()
        .with_tag("latest")
        .start()
        .await
        .unwrap();

    let mut config = init(&container).await;
    let (_, session) = auth(&config.member, &mut config.connection).await;

    let mut payload = HashMap::new();
    payload.insert("name", "Warehouse 1");

    let response = config
        .app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/warehouses")
                .header("cookie", &format!("session={}", session.session))
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 201);

    services::warehouse::find_by_name(payload["name"], &mut config.connection)
        .await
        .unwrap()
        .unwrap();
}

#[tokio::test]
pub async fn store_warehouse_taken() {
    let container = Postgres::default()
        .with_tag("latest")
        .start()
        .await
        .unwrap();

    let mut config = init(&container).await;
    let (_, session) = auth(&config.member, &mut config.connection).await;

    let warehouse_name = "Warehouse 1";

    services::warehouse::insert(
        &StoreWarehouseSchema {
            name: warehouse_name.to_string(),
        },
        &mut config.connection,
    )
    .await
    .unwrap();

    let mut payload = HashMap::new();
    payload.insert("name", warehouse_name);

    let response = config
        .app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/warehouses")
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
pub async fn update_warehouse_success() {
    let container = Postgres::default()
        .with_tag("latest")
        .start()
        .await
        .unwrap();

    let mut config = init(&container).await;
    let (_, session) = auth(&config.member, &mut config.connection).await;

    let warehouse = services::warehouse::insert(
        &StoreWarehouseSchema {
            name: "Warehouse 1".to_string(),
        },
        &mut config.connection,
    )
    .await
    .unwrap();

    let new_warehouse_name = "Warehouse 2";

    let mut payload = HashMap::new();
    payload.insert("name", new_warehouse_name);

    let response = config
        .app
        .oneshot(
            Request::builder()
                .method("PATCH")
                .uri(&format!("/warehouses/{}", warehouse.id.to_string()))
                .header("cookie", &format!("session={}", session.session))
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 204);

    let updated_warehouse = services::warehouse::find(&warehouse.id, &mut config.connection)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(updated_warehouse.name, new_warehouse_name);
}

#[tokio::test]
pub async fn update_warehouse_taken() {
    let container = Postgres::default()
        .with_tag("latest")
        .start()
        .await
        .unwrap();

    let mut config = init(&container).await;
    let (_, session) = auth(&config.member, &mut config.connection).await;

    let warehouse_name = "Warehouse 1";

    services::warehouse::insert(
        &StoreWarehouseSchema {
            name: warehouse_name.to_string(),
        },
        &mut config.connection,
    )
    .await
    .unwrap();

    let warehouse = services::warehouse::insert(
        &StoreWarehouseSchema {
            name: "Warehouse 2".to_string(),
        },
        &mut config.connection,
    )
    .await
    .unwrap();

    let mut payload = HashMap::new();
    payload.insert("name", warehouse_name);

    let response = config
        .app
        .oneshot(
            Request::builder()
                .method("PATCH")
                .uri(&format!("/warehouses/{}", warehouse.id.to_string()))
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
pub async fn delete_warehouse_success() {
    let container = Postgres::default()
        .with_tag("latest")
        .start()
        .await
        .unwrap();

    let mut config = init(&container).await;
    let (_, session) = auth(&config.member, &mut config.connection).await;

    let warehouse = services::warehouse::insert(
        &StoreWarehouseSchema {
            name: "Warehouse 1".to_string(),
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
                .uri(&format!("/warehouses/{}", warehouse.id.to_string()))
                .header("cookie", &format!("session={}", session.session))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 204);

    let warehouse = services::warehouse::find(&warehouse.id, &mut config.connection)
        .await
        .unwrap();
    assert_eq!(warehouse.is_some(), false);
}

#[tokio::test]
pub async fn delete_warehouse_not_found() {
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
                .uri(&format!("/warehouses/{}", Uuid::new_v4().to_string()))
                .header("cookie", &format!("session={}", session.session))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 204);
}

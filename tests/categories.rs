use std::collections::HashMap;

use axum::{body::Body, http::Request};
use http_body_util::BodyExt;
use rumerce::{
    services,
    utils::constants::ROLES,
    validations::{auth::StoreSessionSchema, category::StoreCategorySchema, user::StoreUserSchema},
};
use testcontainers::{runners::AsyncRunner, ImageExt};
use testcontainers_modules::postgres::Postgres;
use tower::ServiceExt;

use common::init;
use uuid::Uuid;

mod common;

#[tokio::test]
pub async fn categories_unauth() {
    let container = Postgres::default()
        .with_tag("latest")
        .start()
        .await
        .unwrap();

    let (app, _) = init(&container).await;

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/categories")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 401);
}

#[tokio::test]
pub async fn categories_no_queries_empty() {
    let container = Postgres::default()
        .with_tag("latest")
        .start()
        .await
        .unwrap();

    let (app, mut connection) = init(&container).await;

    let role = services::role::find_by_name(&ROLES.member, &mut connection)
        .await
        .unwrap()
        .unwrap();

    let user = services::user::insert(
        &StoreUserSchema {
            name: None,
            email: "example@example.com".to_string(),
            role_id: role.id,
        },
        &mut connection,
    )
    .await
    .unwrap();

    let session = services::sessions::insert(
        StoreSessionSchema {
            session: Uuid::new_v4(),
            user_id: user.user.id,
        },
        &mut connection,
    )
    .await
    .unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/categories")
                .header("Cookie", format!("session={}", session.session))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: serde_json::Value = serde_json::from_slice(&body[..]).unwrap();

    match body {
        serde_json::Value::Array(body) => assert_eq!(body.len(), 0),
        _ => panic!("Expected array"),
    };
}

#[tokio::test]
pub async fn categories_no_queries_full() {
    let container = Postgres::default()
        .with_tag("latest")
        .start()
        .await
        .unwrap();

    let (app, mut connection) = init(&container).await;

    let role = services::role::find_by_name(&ROLES.member, &mut connection)
        .await
        .unwrap()
        .unwrap();

    let user = services::user::insert(
        &StoreUserSchema {
            name: None,
            email: "example@example.com".to_string(),
            role_id: role.id,
        },
        &mut connection,
    )
    .await
    .unwrap();

    let session = services::sessions::insert(
        StoreSessionSchema {
            session: Uuid::new_v4(),
            user_id: user.user.id,
        },
        &mut connection,
    )
    .await
    .unwrap();

    let category = services::category::insert(
        &StoreCategorySchema {
            name: "Category 1".to_string(),
        },
        &mut connection,
    )
    .await
    .unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/categories")
                .header("Cookie", format!("session={}", session.session))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: serde_json::Value = serde_json::from_slice(&body[..]).unwrap();

    let body = match body {
        serde_json::Value::Array(body) => body,
        _ => panic!("Expected array"),
    };

    assert_eq!(body.len(), 1);
    assert_eq!(body[0]["name"], category.name);
}

#[tokio::test]
pub async fn categories_show_found() {
    let container = Postgres::default()
        .with_tag("latest")
        .start()
        .await
        .unwrap();

    let (app, mut connection) = init(&container).await;

    let role = services::role::find_by_name(&ROLES.member, &mut connection)
        .await
        .unwrap()
        .unwrap();

    let user = services::user::insert(
        &StoreUserSchema {
            name: None,
            email: "example@example.com".to_string(),
            role_id: role.id,
        },
        &mut connection,
    )
    .await
    .unwrap();

    let session = services::sessions::insert(
        StoreSessionSchema {
            session: Uuid::new_v4(),
            user_id: user.user.id,
        },
        &mut connection,
    )
    .await
    .unwrap();

    let category = services::category::insert(
        &StoreCategorySchema {
            name: "Category 1".to_string(),
        },
        &mut connection,
    )
    .await
    .unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(&format!("/categories/{}", category.id))
                .header("Cookie", format!("session={}", session.session))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: serde_json::Value = serde_json::from_slice(&body[..]).unwrap();

    let body = match body {
        serde_json::Value::Object(body) => body,
        _ => panic!("Expected array"),
    };

    assert_eq!(body["name"], category.name);
}

#[tokio::test]
pub async fn categories_show_not_found() {
    let container = Postgres::default()
        .with_tag("latest")
        .start()
        .await
        .unwrap();

    let (app, mut connection) = init(&container).await;

    let role = services::role::find_by_name(&ROLES.member, &mut connection)
        .await
        .unwrap()
        .unwrap();

    let user = services::user::insert(
        &StoreUserSchema {
            name: None,
            email: "example@example.com".to_string(),
            role_id: role.id,
        },
        &mut connection,
    )
    .await
    .unwrap();

    let session = services::sessions::insert(
        StoreSessionSchema {
            session: Uuid::new_v4(),
            user_id: user.user.id,
        },
        &mut connection,
    )
    .await
    .unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(&format!("/categories/{}", Uuid::new_v4().to_string()))
                .header("Cookie", format!("session={}", session.session))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 404);
}

#[tokio::test]
pub async fn categories_show_unauth() {
    let container = Postgres::default()
        .with_tag("latest")
        .start()
        .await
        .unwrap();

    let (app, _) = init(&container).await;

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(&format!("/categories/{}", Uuid::new_v4().to_string()))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 401);
}

#[tokio::test]
pub async fn store_category_unauth() {
    let container = Postgres::default()
        .with_tag("latest")
        .start()
        .await
        .unwrap();

    let (app, _) = init(&container).await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/categories")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 401);
}

#[tokio::test]
pub async fn store_category_success() {
    let container = Postgres::default()
        .with_tag("latest")
        .start()
        .await
        .unwrap();

    let (app, mut connection) = init(&container).await;

    let role = services::role::find_by_name(&ROLES.member, &mut connection)
        .await
        .unwrap()
        .unwrap();

    let user = services::user::insert(
        &StoreUserSchema {
            name: None,
            email: "example@example.com".to_string(),
            role_id: role.id,
        },
        &mut connection,
    )
    .await
    .unwrap();

    let session = services::sessions::insert(
        StoreSessionSchema {
            session: Uuid::new_v4(),
            user_id: user.user.id,
        },
        &mut connection,
    )
    .await
    .unwrap();

    let mut payload = HashMap::new();
    payload.insert("name", "category 1");

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/categories")
                .header("Cookie", format!("session={}", session.session))
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 201);

    let categories = services::category::all(&mut connection).await.unwrap();
    assert_eq!(categories.len(), 1);
    assert_eq!(categories[0].name, payload["name"]);
}

#[tokio::test]
pub async fn store_category_taken() {
    let container = Postgres::default()
        .with_tag("latest")
        .start()
        .await
        .unwrap();

    let (app, mut connection) = init(&container).await;

    let role = services::role::find_by_name(&ROLES.member, &mut connection)
        .await
        .unwrap()
        .unwrap();

    let user = services::user::insert(
        &StoreUserSchema {
            name: None,
            email: "example@example.com".to_string(),
            role_id: role.id,
        },
        &mut connection,
    )
    .await
    .unwrap();

    let session = services::sessions::insert(
        StoreSessionSchema {
            session: Uuid::new_v4(),
            user_id: user.user.id,
        },
        &mut connection,
    )
    .await
    .unwrap();

    let category = services::category::insert(
        &StoreCategorySchema {
            name: "category 1".to_string(),
        },
        &mut connection,
    )
    .await
    .unwrap();

    let mut payload = HashMap::new();
    payload.insert("name", category.name);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/categories")
                .header("Cookie", format!("session={}", session.session))
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 400);
}

#[tokio::test]
pub async fn delete_category_unauth() {
    let container = Postgres::default()
        .with_tag("latest")
        .start()
        .await
        .unwrap();

    let (app, _) = init(&container).await;

    let response = app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(&format!("/categories/{}", Uuid::new_v4().to_string()))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 401);
}

#[tokio::test]
pub async fn delete_category_success() {
    let container = Postgres::default()
        .with_tag("latest")
        .start()
        .await
        .unwrap();

    let (app, mut connection) = init(&container).await;

    let role = services::role::find_by_name(&ROLES.member, &mut connection)
        .await
        .unwrap()
        .unwrap();

    let user = services::user::insert(
        &StoreUserSchema {
            name: None,
            email: "example@example.com".to_string(),
            role_id: role.id,
        },
        &mut connection,
    )
    .await
    .unwrap();

    let session = services::sessions::insert(
        StoreSessionSchema {
            session: Uuid::new_v4(),
            user_id: user.user.id,
        },
        &mut connection,
    )
    .await
    .unwrap();

    let category = services::category::insert(
        &StoreCategorySchema {
            name: "category 1".to_string(),
        },
        &mut connection,
    )
    .await
    .unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(&format!("/categories/{}", category.id.to_string()))
                .header("Cookie", format!("session={}", session.session))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 204);
}

#[tokio::test]
pub async fn delete_category_not_found() {
    let container = Postgres::default()
        .with_tag("latest")
        .start()
        .await
        .unwrap();

    let (app, mut connection) = init(&container).await;

    let role = services::role::find_by_name(&ROLES.member, &mut connection)
        .await
        .unwrap()
        .unwrap();

    let user = services::user::insert(
        &StoreUserSchema {
            name: None,
            email: "example@example.com".to_string(),
            role_id: role.id,
        },
        &mut connection,
    )
    .await
    .unwrap();

    let session = services::sessions::insert(
        StoreSessionSchema {
            session: Uuid::new_v4(),
            user_id: user.user.id,
        },
        &mut connection,
    )
    .await
    .unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(&format!("/categories/{}", Uuid::new_v4().to_string()))
                .header("Cookie", format!("session={}", session.session))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 204);
}

#[tokio::test]
pub async fn update_category_unauth() {
    let container = Postgres::default()
        .with_tag("latest")
        .start()
        .await
        .unwrap();

    let (app, _) = init(&container).await;

    let response = app
        .oneshot(
            Request::builder()
                .method("PATCH")
                .uri(&format!("/categories/{}", Uuid::new_v4().to_string()))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 401);
}

#[tokio::test]
pub async fn update_category_not_found() {
    let container = Postgres::default()
        .with_tag("latest")
        .start()
        .await
        .unwrap();

    let (app, mut connection) = init(&container).await;

    let role = services::role::find_by_name(&ROLES.member, &mut connection)
        .await
        .unwrap()
        .unwrap();

    let user = services::user::insert(
        &StoreUserSchema {
            name: None,
            email: "example@example.com".to_string(),
            role_id: role.id,
        },
        &mut connection,
    )
    .await
    .unwrap();

    let session = services::sessions::insert(
        StoreSessionSchema {
            session: Uuid::new_v4(),
            user_id: user.user.id,
        },
        &mut connection,
    )
    .await
    .unwrap();

    let mut payload = HashMap::new();
    payload.insert("name", "category 1");

    let response = app
        .oneshot(
            Request::builder()
                .method("PATCH")
                .uri(&format!("/categories/{}", Uuid::new_v4().to_string()))
                .header("Cookie", format!("session={}", session.session))
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 404);
}

#[tokio::test]
pub async fn update_category_success() {
    let container = Postgres::default()
        .with_tag("latest")
        .start()
        .await
        .unwrap();

    let (app, mut connection) = init(&container).await;

    let role = services::role::find_by_name(&ROLES.member, &mut connection)
        .await
        .unwrap()
        .unwrap();

    let user = services::user::insert(
        &StoreUserSchema {
            name: None,
            email: "example@example.com".to_string(),
            role_id: role.id,
        },
        &mut connection,
    )
    .await
    .unwrap();

    let session = services::sessions::insert(
        StoreSessionSchema {
            session: Uuid::new_v4(),
            user_id: user.user.id,
        },
        &mut connection,
    )
    .await
    .unwrap();

    let category = services::category::insert(
        &StoreCategorySchema {
            name: "category 1".to_string(),
        },
        &mut connection,
    )
    .await
    .unwrap();

    let new_category = "Category 2";

    let mut payload = HashMap::new();
    payload.insert("name", new_category);

    let response = app
        .oneshot(
            Request::builder()
                .method("PATCH")
                .uri(&format!("/categories/{}", category.id))
                .header("Cookie", format!("session={}", session.session))
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 200);

    let category = services::category::find(&category.id, &mut connection)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(category.name, new_category);
}

#[tokio::test]
pub async fn update_category_same_category() {
    let container = Postgres::default()
        .with_tag("latest")
        .start()
        .await
        .unwrap();

    let (app, mut connection) = init(&container).await;

    let role = services::role::find_by_name(&ROLES.member, &mut connection)
        .await
        .unwrap()
        .unwrap();

    let user = services::user::insert(
        &StoreUserSchema {
            name: None,
            email: "example@example.com".to_string(),
            role_id: role.id,
        },
        &mut connection,
    )
    .await
    .unwrap();

    let session = services::sessions::insert(
        StoreSessionSchema {
            session: Uuid::new_v4(),
            user_id: user.user.id,
        },
        &mut connection,
    )
    .await
    .unwrap();

    let category = services::category::insert(
        &StoreCategorySchema {
            name: "category 1".to_string(),
        },
        &mut connection,
    )
    .await
    .unwrap();

    let mut payload = HashMap::new();
    payload.insert("name", category.name);

    let response = app
        .oneshot(
            Request::builder()
                .method("PATCH")
                .uri(&format!("/categories/{}", category.id))
                .header("Cookie", format!("session={}", session.session))
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
}

#[tokio::test]
pub async fn update_category_taken() {
    let container = Postgres::default()
        .with_tag("latest")
        .start()
        .await
        .unwrap();

    let (app, mut connection) = init(&container).await;

    let role = services::role::find_by_name(&ROLES.member, &mut connection)
        .await
        .unwrap()
        .unwrap();

    let user = services::user::insert(
        &StoreUserSchema {
            name: None,
            email: "example@example.com".to_string(),
            role_id: role.id,
        },
        &mut connection,
    )
    .await
    .unwrap();

    let session = services::sessions::insert(
        StoreSessionSchema {
            session: Uuid::new_v4(),
            user_id: user.user.id,
        },
        &mut connection,
    )
    .await
    .unwrap();

    let category = services::category::insert(
        &StoreCategorySchema {
            name: "category 1".to_string(),
        },
        &mut connection,
    )
    .await
    .unwrap();

    let second_category = services::category::insert(
        &StoreCategorySchema {
            name: "category 2".to_string(),
        },
        &mut connection,
    )
    .await
    .unwrap();

    let mut payload = HashMap::new();
    payload.insert("name", second_category.name);

    let response = app
        .oneshot(
            Request::builder()
                .method("PATCH")
                .uri(&format!("/categories/{}", category.id))
                .header("Cookie", format!("session={}", session.session))
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 403);
}

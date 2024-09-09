use uuid::Uuid;

use crate::{
    models::warehouse::Warehouse, utils::db::DB, validations::warehouse::StoreWarehouseSchema,
};

pub async fn all(db: &DB) -> Vec<Warehouse> {
    sqlx::query_as!(Warehouse, "SELECT * FROM warehouses")
        .fetch_all(db)
        .await
        .unwrap()
}

pub async fn find(id: &Uuid, db: &DB) -> Option<Warehouse> {
    sqlx::query_as!(Warehouse, "SELECT * FROM warehouses WHERE id = $1", id)
        .fetch_optional(db)
        .await
        .unwrap()
}

pub async fn insert(input: &StoreWarehouseSchema, db: &DB) {
    sqlx::query!("INSERT INTO warehouses(name) VALUES ($1)", input.name)
        .execute(db)
        .await
        .unwrap();
}

pub async fn update(id: &Uuid, input: &StoreWarehouseSchema, db: &DB) {
    sqlx::query!(
        "UPDATE warehouses SET name = $1 WHERE id = $2",
        input.name,
        id
    )
    .execute(db)
    .await
    .unwrap();
}

pub async fn delete(id: &Uuid, db: &DB) {
    sqlx::query!("DELETE FROM warehouses WHERE id = $1", id)
        .execute(db)
        .await
        .unwrap();
}

use sqlx::postgres::PgQueryResult;
use uuid::Uuid;

use crate::{
    models::warehouse::Warehouse, utils::db::DB, validations::warehouse::StoreWarehouseSchema,
};

pub async fn all(db: &DB) -> Result<Vec<Warehouse>, sqlx::Error> {
    sqlx::query_as!(Warehouse, "SELECT * FROM warehouses")
        .fetch_all(db)
        .await
}

pub async fn find(id: &Uuid, db: &DB) -> Result<Option<Warehouse>, sqlx::Error> {
    sqlx::query_as!(Warehouse, "SELECT * FROM warehouses WHERE id = $1", id)
        .fetch_optional(db)
        .await
}

pub async fn insert(input: &StoreWarehouseSchema, db: &DB) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!("INSERT INTO warehouses(name) VALUES ($1)", input.name)
        .execute(db)
        .await
}

pub async fn update(
    id: &Uuid,
    input: &StoreWarehouseSchema,
    db: &DB,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        "UPDATE warehouses SET name = $1 WHERE id = $2",
        input.name,
        id
    )
    .execute(db)
    .await
}

pub async fn delete(id: &Uuid, db: &DB) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!("DELETE FROM warehouses WHERE id = $1", id)
        .execute(db)
        .await
}

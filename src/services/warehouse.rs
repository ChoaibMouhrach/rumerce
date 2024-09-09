use sqlx::{postgres::PgQueryResult, PgConnection};
use uuid::Uuid;

use crate::{models::warehouse::Warehouse, validations::warehouse::StoreWarehouseSchema};

pub async fn all(db: &mut PgConnection) -> Result<Vec<Warehouse>, sqlx::Error> {
    sqlx::query_as!(Warehouse, "SELECT * FROM warehouses")
        .fetch_all(&mut *db)
        .await
}

pub async fn find(id: &Uuid, db: &mut PgConnection) -> Result<Option<Warehouse>, sqlx::Error> {
    sqlx::query_as!(Warehouse, "SELECT * FROM warehouses WHERE id = $1", id)
        .fetch_optional(&mut *db)
        .await
}

pub async fn insert(
    input: &StoreWarehouseSchema,
    db: &mut PgConnection,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!("INSERT INTO warehouses(name) VALUES ($1)", input.name)
        .execute(&mut *db)
        .await
}

pub async fn update(
    id: &Uuid,
    input: &StoreWarehouseSchema,
    db: &mut PgConnection,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        "UPDATE warehouses SET name = $1 WHERE id = $2",
        input.name,
        id
    )
    .execute(&mut *db)
    .await
}

pub async fn delete(id: &Uuid, db: &mut PgConnection) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!("DELETE FROM warehouses WHERE id = $1", id)
        .execute(&mut *db)
        .await
}

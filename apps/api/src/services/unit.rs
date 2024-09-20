use crate::{models::unit::Unit, validations::unit::StoreUnitSchema};
use sqlx::{postgres::PgQueryResult, PgConnection};
use uuid::Uuid;

pub async fn all(db: &mut PgConnection) -> Result<Vec<Unit>, sqlx::Error> {
    sqlx::query_as!(Unit, "SELECT * FROM units")
        .fetch_all(&mut *db)
        .await
}

pub async fn find(id: &Uuid, db: &mut PgConnection) -> Result<Option<Unit>, sqlx::Error> {
    sqlx::query_as!(Unit, "SELECT * FROM units WHERE id = $1", id)
        .fetch_optional(&mut *db)
        .await
}

pub async fn find_by_name(name: &str, db: &mut PgConnection) -> Result<Option<Unit>, sqlx::Error> {
    sqlx::query_as!(Unit, "SELECT * FROM units WHERE name = $1", name)
        .fetch_optional(&mut *db)
        .await
}

pub async fn insert(input: &StoreUnitSchema, db: &mut PgConnection) -> Result<Unit, sqlx::Error> {
    sqlx::query_as!(
        Unit,
        "INSERT INTO units(name) VALUES ($1) RETURNING *",
        input.name
    )
    .fetch_one(&mut *db)
    .await
}

pub async fn update(
    id: &Uuid,
    input: &StoreUnitSchema,
    db: &mut PgConnection,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!("UPDATE units SET name = $1 WHERE id = $2", input.name, id)
        .execute(&mut *db)
        .await
}

pub async fn destroy(id: &Uuid, db: &mut PgConnection) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!("DELETE FROM units WHERE id = $1", id)
        .execute(&mut *db)
        .await
}

use crate::{
    models::unit::Unit,
    utils::db::DB,
    validations::unit::{StoreUnitSchema, UpdateUnitSchema},
};
use sqlx::postgres::PgQueryResult;
use uuid::Uuid;

pub async fn all(db: &DB) -> Result<Vec<Unit>, sqlx::Error> {
    sqlx::query_as!(Unit, "SELECT * FROM units")
        .fetch_all(db)
        .await
}

pub async fn find(id: &Uuid, db: &DB) -> Result<Option<Unit>, sqlx::Error> {
    sqlx::query_as!(Unit, "SELECT * FROM units WHERE id = $1", id)
        .fetch_optional(db)
        .await
}

pub async fn insert(input: &StoreUnitSchema, db: &DB) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!("INSERT INTO units(name) VALUES ($1)", input.name)
        .execute(db)
        .await
}

pub async fn update(
    id: &Uuid,
    input: &UpdateUnitSchema,
    db: &DB,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!("UPDATE units SET name = $1 WHERE id = $2", input.name, id)
        .execute(db)
        .await
}

pub async fn destroy(id: &Uuid, db: &DB) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!("DELETE FROM units WHERE id = $1", id)
        .execute(db)
        .await
}

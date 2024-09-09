use crate::{
    models::role::Role,
    utils::db::DB,
    validations::role::{StoreRoleSchema, UpdateRoleSchema},
};
use sqlx::postgres::PgQueryResult;
use uuid::Uuid;

pub async fn all(db: &DB) -> Result<Vec<Role>, sqlx::Error> {
    sqlx::query_as!(Role, "SELECT * FROM roles")
        .fetch_all(db)
        .await
}

pub async fn find(id: &Uuid, db: &DB) -> Result<Option<Role>, sqlx::Error> {
    sqlx::query_as!(Role, "SELECT * FROM roles WHERE id = $1", id)
        .fetch_optional(db)
        .await
}

pub async fn insert(input: &StoreRoleSchema, db: &DB) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!("INSERT INTO roles(name) VALUES ($1)", input.name)
        .execute(db)
        .await
}

pub async fn update(
    id: &Uuid,
    input: &UpdateRoleSchema,
    db: &DB,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!("UPDATE roles SET name = $1 WHERE id = $2", input.name, id)
        .execute(db)
        .await
}

pub async fn destroy(id: &Uuid, db: &DB) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!("DELETE FROM roles WHERE id = $1", id)
        .execute(db)
        .await
}

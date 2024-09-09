use crate::{
    models::role::Role,
    validations::role::{StoreRoleSchema, UpdateRoleSchema},
};
use sqlx::{postgres::PgQueryResult, PgConnection};
use uuid::Uuid;

pub async fn all(db: &mut PgConnection) -> Result<Vec<Role>, sqlx::Error> {
    sqlx::query_as!(Role, "SELECT * FROM roles")
        .fetch_all(&mut *db)
        .await
}

pub async fn find(id: &Uuid, db: &mut PgConnection) -> Result<Option<Role>, sqlx::Error> {
    sqlx::query_as!(Role, "SELECT * FROM roles WHERE id = $1", id)
        .fetch_optional(&mut *db)
        .await
}

pub async fn insert(
    input: &StoreRoleSchema,
    db: &mut PgConnection,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!("INSERT INTO roles(name) VALUES ($1)", input.name)
        .execute(&mut *db)
        .await
}

pub async fn update(
    id: &Uuid,
    input: &UpdateRoleSchema,
    db: &mut PgConnection,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!("UPDATE roles SET name = $1 WHERE id = $2", input.name, id)
        .execute(&mut *db)
        .await
}

pub async fn destroy(id: &Uuid, db: &mut PgConnection) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!("DELETE FROM roles WHERE id = $1", id)
        .execute(&mut *db)
        .await
}

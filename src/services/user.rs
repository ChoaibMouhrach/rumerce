use sqlx::postgres::PgQueryResult;
use uuid::Uuid;

use crate::{
    models::{
        role::Role,
        user::{PopulatedUser, User},
    },
    utils::db::DB,
    validations::user::{StoreUserSchema, UpdateUserSchema},
};

pub async fn all(db: &DB) -> Result<Vec<PopulatedUser>, sqlx::Error> {
    sqlx::query_as!(
        PopulatedUser,
        r#"
            SELECT 
                (users.id, users.name, users.email, users.role_id, users.created_at) as "user!: User",
                (roles.id, roles.name, roles.created_at) as "role!: Role"
            FROM users 
            JOIN roles ON roles.id = users.role_id
        "#
    )
    .fetch_all(db)
    .await
}

pub async fn find(id: &Uuid, db: &DB) -> Result<Option<PopulatedUser>, sqlx::Error> {
    sqlx::query_as!(
        PopulatedUser,
        r#"
            SELECT 
                (users.id, users.name, users.email, users.role_id, users.created_at) as "user!: User",
                (roles.id, roles.name, roles.created_at) as "role!: Role"
            FROM users 
            JOIN roles ON roles.id = users.role_id
            WHERE users.id = $1
        "#,
        id
        )
        .fetch_optional(db)
        .await
}

pub async fn insert(input: &StoreUserSchema, db: &DB) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        "INSERT INTO users(name, email, role_id) VALUES($1, $2, $3)",
        input.name,
        input.email,
        input.role_id,
    )
    .execute(db)
    .await
}

pub async fn update(
    id: &Uuid,
    input: &UpdateUserSchema,
    db: &DB,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        "UPDATE users SET name = $1, email = $2 WHERE id = $3",
        input.name,
        input.email,
        id
    )
    .execute(db)
    .await
}

pub async fn destroy(id: &Uuid, db: &DB) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!("DELETE FROM users WHERE id = $1", id)
        .execute(db)
        .await
}

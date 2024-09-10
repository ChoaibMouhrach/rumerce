use sqlx::{postgres::PgQueryResult, PgConnection};
use uuid::Uuid;

use crate::{
    models::{
        role::Role,
        user::{PopulatedUser, User},
    },
    validations::user::{StoreUserSchema, UpdateUserSchema},
};

pub async fn all(db: &mut PgConnection) -> Result<Vec<PopulatedUser>, sqlx::Error> {
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
    .fetch_all(&mut *db)
    .await
}

pub async fn find(id: &Uuid, db: &mut PgConnection) -> Result<Option<PopulatedUser>, sqlx::Error> {
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
        .fetch_optional(&mut *db)
        .await
}

pub async fn find_by_email(
    email: &str,
    db: &mut PgConnection,
) -> Result<Option<PopulatedUser>, sqlx::Error> {
    sqlx::query_as!(
        PopulatedUser,
        r#"
            SELECT 
                (users.id, users.name, users.email, users.role_id, users.created_at) as "user!: User",
                (roles.id, roles.name, roles.created_at) as "role!: Role"
            FROM users 
            JOIN roles ON roles.id = users.role_id
            WHERE users.email = $1
        "#,
        email
        )
        .fetch_optional(&mut *db)
        .await
}

pub async fn insert(
    input: &StoreUserSchema,
    db: &mut PgConnection,
) -> Result<PopulatedUser, sqlx::Error> {
    sqlx::query_as!(
        PopulatedUser,
        r#"
            WITH new_user AS (
                INSERT INTO users(name, email, role_id) VALUES($1, $2, $3) RETURNING *
            )
            SELECT 
                (new_user.id, new_user.name, new_user.email, new_user.role_id, new_user.created_at) as "user!: User",
                (roles.id, roles.name, roles.created_at) as "role!: Role"
            FROM new_user
            JOIN roles ON roles.id = new_user.role_id
        "#,
        input.name,
        input.email,
        input.role_id,
    )
    .fetch_one(&mut *db)
    .await
}

pub async fn update(
    id: &Uuid,
    input: &UpdateUserSchema,
    db: &mut PgConnection,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        "UPDATE users SET name = $1, email = $2 WHERE id = $3",
        input.name,
        input.email,
        id
    )
    .execute(&mut *db)
    .await
}

pub async fn destroy(id: &Uuid, db: &mut PgConnection) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!("DELETE FROM users WHERE id = $1", id)
        .execute(&mut *db)
        .await
}

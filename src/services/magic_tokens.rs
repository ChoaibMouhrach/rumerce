use sqlx::{postgres::PgQueryResult, PgConnection};
use uuid::Uuid;

use crate::{
    models::{
        magic_tokens::{MagicToken, PopulatedMagicToken},
        user::User,
    },
    validations::magic_tokens::StoreMagicTokenSchema,
};

pub async fn all(connection: &mut PgConnection) -> Result<Vec<PopulatedMagicToken>, sqlx::Error> {
    sqlx::query_as!(
        PopulatedMagicToken,
        r#"
      SELECT 
        (users.id, users.name, users.email, users.role_id, users.created_at) as "user!: User",
        (magic_tokens.id, magic_tokens.token, magic_tokens.user_id, magic_tokens.expires_at) as "token!: MagicToken"
      FROM
        magic_tokens
      JOIN users ON users.id = magic_tokens.user_id
    "#
    ).fetch_all(connection).await
}

pub async fn find(
    id: &Uuid,
    connection: &mut PgConnection,
) -> Result<Option<PopulatedMagicToken>, sqlx::Error> {
    sqlx::query_as!(
        PopulatedMagicToken,
        r#"
      SELECT 
        (users.id, users.name, users.email, users.role_id, users.created_at) as "user!: User",
        (magic_tokens.id, magic_tokens.token, magic_tokens.user_id, magic_tokens.expires_at) as "token!: MagicToken"
      FROM
        magic_tokens
      JOIN users ON users.id = magic_tokens.user_id
      WHERE magic_tokens.id = $1
    "#,
    id
    ).fetch_optional(connection).await
}

pub async fn find_by_token(
    token: &Uuid,
    connection: &mut PgConnection,
) -> Result<Option<PopulatedMagicToken>, sqlx::Error> {
    sqlx::query_as!(
        PopulatedMagicToken,
        r#"
      SELECT 
        (users.id, users.name, users.email, users.role_id, users.created_at) as "user!: User",
        (magic_tokens.id, magic_tokens.token, magic_tokens.user_id, magic_tokens.expires_at) as "token!: MagicToken"
      FROM
        magic_tokens
      JOIN users ON users.id = magic_tokens.user_id
      WHERE magic_tokens.token = $1
    "#,
    token
    ).fetch_optional(connection).await
}

pub async fn find_by_user(
    user_id: &Uuid,
    connection: &mut PgConnection,
) -> Result<Option<PopulatedMagicToken>, sqlx::Error> {
    sqlx::query_as!(
        PopulatedMagicToken,
        r#"
          SELECT 
            (users.id, users.name, users.email, users.role_id, users.created_at) as "user!: User",
            (magic_tokens.id, magic_tokens.token, magic_tokens.user_id, magic_tokens.expires_at) as "token!: MagicToken"
          FROM
            magic_tokens
          JOIN users ON users.id = magic_tokens.user_id
          WHERE magic_tokens.user_id = $1
        "#,
    user_id
    ).fetch_optional(connection).await
}

pub async fn insert(
    input: &StoreMagicTokenSchema,
    connection: &mut PgConnection,
) -> Result<PopulatedMagicToken, sqlx::Error> {
    sqlx::query_as!(
        PopulatedMagicToken,
        r#"
        WITH new_token AS (
          INSERT INTO magic_tokens(token, user_id,expires_at) VALUES ($1, $2, $3) RETURNING *
        )
        SELECT 
          (users.id, users.name, users.email, users.role_id, users.created_at) as "user!: User",
          (new_token.id, new_token.token, new_token.user_id, new_token.expires_at) as "token!: MagicToken"
        FROM
          new_token
        JOIN users ON users.id = new_token.user_id
    "#,
    input.token,
    input.user_id,
    input.expires_at
    ).fetch_one(connection).await
}

pub async fn destroy(
    id: &Uuid,
    connection: &mut PgConnection,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(r#"DELETE FROM magic_tokens WHERE id = $1"#, id)
        .execute(connection)
        .await
}

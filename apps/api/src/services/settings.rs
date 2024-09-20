use crate::{models::settings::Settings, validations::settings::StoreSettingsSchema};
use sqlx::{postgres::PgQueryResult, Error, PgConnection};
use uuid::Uuid;

pub async fn all(connection: &mut PgConnection) -> Result<Vec<Settings>, Error> {
    sqlx::query_as!(Settings, "SELECT * FROM settings")
        .fetch_all(connection)
        .await
}

pub async fn find(id: &Uuid, connection: &mut PgConnection) -> Result<Option<Settings>, Error> {
    sqlx::query_as!(Settings, "SELECT * FROM settings WHERE id = $1", id)
        .fetch_optional(connection)
        .await
}

pub async fn find_by_key(
    key: &str,
    connection: &mut PgConnection,
) -> Result<Option<Settings>, Error> {
    sqlx::query_as!(Settings, "SELECT * FROM settings WHERE key = $1", key)
        .fetch_optional(connection)
        .await
}

pub async fn insert(
    input: &StoreSettingsSchema,
    connection: &mut PgConnection,
) -> Result<Option<Settings>, Error> {
    sqlx::query_as!(
        Settings,
        "INSERT INTO settings (key, value) VALUES ($1, $2) RETURNING *",
        input.key,
        input.value
    )
    .fetch_optional(connection)
    .await
}

pub async fn update(
    id: &Uuid,
    input: &StoreSettingsSchema,
    connection: &mut PgConnection,
) -> Result<PgQueryResult, Error> {
    sqlx::query!(
        "UPDATE settings SET key = $2, value = $3 WHERE id = $1",
        id,
        input.key,
        input.value
    )
    .execute(connection)
    .await
}

pub async fn destroy(id: &Uuid, connection: &mut PgConnection) -> Result<PgQueryResult, Error> {
    sqlx::query!("DELETE FROM settings WHERE id = $1", id)
        .execute(connection)
        .await
}

use crate::{models::category::Category, validations::category::StoreCategorySchema};
use sqlx::{postgres::PgQueryResult, PgConnection};
use uuid::Uuid;

pub async fn all(db: &mut PgConnection) -> Result<Vec<Category>, sqlx::Error> {
    sqlx::query_as!(Category, "SELECT * FROM categories")
        .fetch_all(&mut *db)
        .await
}

pub async fn find(id: &Uuid, db: &mut PgConnection) -> Result<Option<Category>, sqlx::Error> {
    sqlx::query_as!(Category, "SELECT * FROM categories WHERE id = $1", id)
        .fetch_optional(&mut *db)
        .await
}

pub async fn insert(
    input: &StoreCategorySchema,
    db: &mut PgConnection,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!("INSERT INTO categories(name) VALUES ($1)", input.name)
        .execute(&mut *db)
        .await
}

pub async fn update(
    id: &Uuid,
    input: &StoreCategorySchema,
    db: &mut PgConnection,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        "UPDATE categories SET name = $1 WHERE id = $2",
        input.name,
        id
    )
    .execute(&mut *db)
    .await
}

pub async fn destroy(id: &Uuid, db: &mut PgConnection) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!("DELETE FROM categories WHERE id = $1", id)
        .execute(&mut *db)
        .await
}

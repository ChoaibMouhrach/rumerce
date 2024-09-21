use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::{postgres::PgQueryResult, prelude::FromRow, PgConnection};
use uuid::Uuid;

use crate::validations::category::StoreCategorySchema;

#[derive(FromRow, Serialize, sqlx::Type, Debug, Clone)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub created_at: NaiveDateTime,
}

impl Category {
    pub async fn all(connection: &mut PgConnection) -> Result<Vec<Category>, sqlx::Error> {
        sqlx::query_as!(Category, "SELECT * FROM categories")
            .fetch_all(connection)
            .await
    }

    pub async fn find(
        id: &Uuid,
        connection: &mut PgConnection,
    ) -> Result<Option<Category>, sqlx::Error> {
        sqlx::query_as!(Category, "SELECT * FROM categories WHERE id = $1", id)
            .fetch_optional(connection)
            .await
    }

    pub async fn find_by_name(
        name: &str,
        connection: &mut PgConnection,
    ) -> Result<Option<Category>, sqlx::Error> {
        sqlx::query_as!(Category, "SELECT * FROM categories WHERE name = $1", name)
            .fetch_optional(connection)
            .await
    }

    pub async fn insert<'a>(
        input: StoreCategorySchema<'a>,
        connection: &mut PgConnection,
    ) -> Result<Category, sqlx::Error> {
        sqlx::query_as!(
            Category,
            "INSERT INTO categories(name) VALUES ($1) RETURNING *",
            input.name
        )
        .fetch_one(connection)
        .await
    }

    pub async fn insert_many<'a>(
        input: Vec<StoreCategorySchema<'a>>,
        connection: &mut PgConnection,
    ) -> Result<Category, sqlx::Error> {
        sqlx::query_as!(
            Category,
            "INSERT INTO categories(name) SELECT * FROM UNNEST($1::TEXT[]) RETURNING *",
            &input
                .into_iter()
                .map(|category| category.name.to_string())
                .collect::<Vec<_>>()
        )
        .fetch_one(connection)
        .await
    }

    pub async fn update<'a>(
        id: &Uuid,
        input: StoreCategorySchema<'a>,
        connection: &mut PgConnection,
    ) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query!(
            "UPDATE categories SET name = $2 WHERE id = $1",
            id,
            input.name
        )
        .execute(connection)
        .await
    }

    pub async fn destroy(
        id: &Uuid,
        connection: &mut PgConnection,
    ) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query!("DELETE FROM categories WHERE id = $1", id,)
            .execute(connection)
            .await
    }

    pub async fn save(&self, connection: &mut PgConnection) -> Result<PgQueryResult, sqlx::Error> {
        Category::update(
            &self.id,
            StoreCategorySchema { name: &self.name },
            connection,
        )
        .await
    }

    pub async fn delete(
        &self,
        connection: &mut PgConnection,
    ) -> Result<PgQueryResult, sqlx::Error> {
        Category::destroy(&self.id, connection).await
    }
}

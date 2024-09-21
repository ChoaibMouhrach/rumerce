use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::{postgres::PgQueryResult, prelude::FromRow, PgConnection};
use uuid::Uuid;

use crate::validations::unit::StoreUnitSchema;

#[derive(FromRow, Serialize, sqlx::Type, Debug, Clone)]
pub struct Unit {
    pub id: Uuid,
    pub name: String,
    pub created_at: NaiveDateTime,
}

impl Unit {
    pub async fn all(connection: &mut PgConnection) -> Result<Vec<Unit>, sqlx::Error> {
        sqlx::query_as!(Unit, "SELECT * FROM units")
            .fetch_all(connection)
            .await
    }

    pub async fn find(
        id: &Uuid,
        connection: &mut PgConnection,
    ) -> Result<Option<Unit>, sqlx::Error> {
        sqlx::query_as!(Unit, "SELECT * FROM units WHERE id = $1", id)
            .fetch_optional(connection)
            .await
    }

    pub async fn find_by_name(
        name: &str,
        connection: &mut PgConnection,
    ) -> Result<Option<Unit>, sqlx::Error> {
        sqlx::query_as!(Unit, "SELECT * FROM units WHERE name = $1", name)
            .fetch_optional(connection)
            .await
    }

    pub async fn insert<'a>(
        input: StoreUnitSchema<'a>,
        connection: &mut PgConnection,
    ) -> Result<Unit, sqlx::Error> {
        sqlx::query_as!(
            Unit,
            "INSERT INTO units (name) VALUES ($1) RETURNING *",
            input.name
        )
        .fetch_one(connection)
        .await
    }

    pub async fn insert_many<'a>(
        input: Vec<StoreUnitSchema<'a>>,
        connection: &mut PgConnection,
    ) -> Result<Vec<Unit>, sqlx::Error> {
        sqlx::query_as!(
            Unit,
            "INSERT INTO units (name) SELECT * FROM UNNEST($1::TEXT[]) RETURNING *",
            &input
                .into_iter()
                .map(|unit| unit.name.to_string())
                .collect::<Vec<_>>()
        )
        .fetch_all(connection)
        .await
    }

    pub async fn update<'a>(
        id: &Uuid,
        input: StoreUnitSchema<'a>,
        connection: &mut PgConnection,
    ) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query!("UPDATE units SET name = $2 WHERE id = $1", id, input.name)
            .execute(connection)
            .await
    }

    pub async fn destroy(
        id: &Uuid,
        connection: &mut PgConnection,
    ) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query!("DELETE FROM units WHERE id = $1", id)
            .execute(connection)
            .await
    }

    pub async fn save(&self, connection: &mut PgConnection) -> Result<PgQueryResult, sqlx::Error> {
        Unit::update(&self.id, StoreUnitSchema { name: &self.name }, connection).await
    }

    pub async fn delete(
        &self,
        connection: &mut PgConnection,
    ) -> Result<PgQueryResult, sqlx::Error> {
        Unit::destroy(&self.id, connection).await
    }
}

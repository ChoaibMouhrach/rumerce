use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::{postgres::PgQueryResult, prelude::FromRow, PgConnection};
use uuid::Uuid;

use crate::validations::warehouse::StoreWarehouseSchema;

#[derive(FromRow, Serialize)]
pub struct Warehouse {
    pub id: Uuid,
    pub name: String,
    pub created_at: NaiveDateTime,
}

impl Warehouse {
    pub async fn all(connection: &mut PgConnection) -> Result<Vec<Warehouse>, sqlx::Error> {
        sqlx::query_as!(Warehouse, "SELECT * FROM warehouses")
            .fetch_all(connection)
            .await
    }

    pub async fn find(
        id: &Uuid,
        connection: &mut PgConnection,
    ) -> Result<Option<Warehouse>, sqlx::Error> {
        sqlx::query_as!(Warehouse, "SELECT * FROM warehouses WHERE id = $1", id)
            .fetch_optional(connection)
            .await
    }

    pub async fn find_by_name(
        name: &str,
        connection: &mut PgConnection,
    ) -> Result<Option<Warehouse>, sqlx::Error> {
        sqlx::query_as!(Warehouse, "SELECT * FROM warehouses WHERE name = $1", name)
            .fetch_optional(connection)
            .await
    }

    pub async fn insert<'a>(
        input: StoreWarehouseSchema<'a>,
        connection: &mut PgConnection,
    ) -> Result<Warehouse, sqlx::Error> {
        sqlx::query_as!(
            Warehouse,
            "INSERT INTO warehouses (name) VALUES ($1) RETURNING *",
            input.name
        )
        .fetch_one(connection)
        .await
    }

    pub async fn insert_many<'a>(
        input: Vec<StoreWarehouseSchema<'a>>,
        connection: &mut PgConnection,
    ) -> Result<Warehouse, sqlx::Error> {
        sqlx::query_as!(
            Warehouse,
            "INSERT INTO warehouses (name) SELECT * FROM UNNEST($1::TEXT[]) RETURNING *",
            &input
                .into_iter()
                .map(|warehouse| warehouse.name.to_string())
                .collect::<Vec<_>>()
        )
        .fetch_one(connection)
        .await
    }

    pub async fn update<'a>(
        id: &Uuid,
        input: StoreWarehouseSchema<'a>,
        connection: &mut PgConnection,
    ) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query!(
            "UPDATE warehouses SET name = $2 WHERE id = $1",
            id,
            input.name
        )
        .execute(connection)
        .await
    }

    pub async fn destroy<'a>(
        id: &Uuid,
        connection: &mut PgConnection,
    ) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query!("DELETE FROM warehouses WHERE id = $1", id)
            .execute(connection)
            .await
    }

    pub async fn save(self, connection: &mut PgConnection) -> Result<PgQueryResult, sqlx::Error> {
        Warehouse::update(
            &self.id,
            StoreWarehouseSchema { name: &self.name },
            connection,
        )
        .await
    }

    pub async fn delete(self, connection: &mut PgConnection) -> Result<PgQueryResult, sqlx::Error> {
        Warehouse::destroy(&self.id, connection).await
    }
}

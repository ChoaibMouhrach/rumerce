use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::{postgres::PgQueryResult, prelude::FromRow, PgConnection};
use uuid::Uuid;

use crate::validations::user::StoreUserSchema;

use super::role::Role;

#[derive(Serialize, FromRow)]
pub struct PopulatedUser {
    pub user: User,
    pub role: Role,
}

#[derive(Serialize, FromRow, sqlx::Type, Clone)]
pub struct User {
    pub id: Uuid,
    pub name: Option<String>,
    pub email: String,
    pub role_id: Uuid,
    pub created_at: NaiveDateTime,
}

impl User {
    pub async fn all(connection: &mut PgConnection) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as!(User, "SELECT * FROM users")
            .fetch_all(connection)
            .await
    }

    pub async fn find(
        id: &Uuid,
        connection: &mut PgConnection,
    ) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
            .fetch_optional(connection)
            .await
    }

    pub async fn find_by_email(
        name: &str,
        connection: &mut PgConnection,
    ) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as!(User, "SELECT * FROM users WHERE name = $1", name)
            .fetch_optional(connection)
            .await
    }

    pub async fn insert<'a>(
        input: StoreUserSchema<'a>,
        connection: &mut PgConnection,
    ) -> Result<User, sqlx::Error> {
        sqlx::query_as!(
            User,
            "INSERT INTO users(name, email, role_id) VALUES ($1, $2, $3) RETURNING *",
            input.name,
            input.email,
            input.role_id
        )
        .fetch_one(connection)
        .await
    }

    pub async fn insert_many<'a>(
        input: Vec<StoreUserSchema<'a>>,
        connection: &mut PgConnection,
    ) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as!(
            User,
            "INSERT INTO users(name, email, role_id) SELECT * FROM UNNEST($1::TEXT[], $2:: TEXT[], $3::UUID[]) RETURNING *",
            &input.iter().map(|user| match user.name {
                Some(name) => Some(name.to_string()),
                None => None
            } ).collect::<Vec<_>>() as &[Option<String>],
            &input.iter().map(|user| user.email.to_string()).collect::<Vec<_>>(),
            &input.iter().map(|user| user.role_id).collect::<Vec<_>>()
        )
        .fetch_all(connection)
        .await
    }

    pub async fn update<'a>(
        id: &Uuid,
        input: StoreUserSchema<'a>,
        connection: &mut PgConnection,
    ) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query!(
            "UPDATE users SET name = $2, email = $3, role_id = $4 WHERE id = $1",
            id,
            input.name,
            input.email,
            input.role_id
        )
        .execute(connection)
        .await
    }

    pub async fn destroy(
        id: &Uuid,
        connection: &mut PgConnection,
    ) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query!("DELETE FROM users WHERE id = $1", id,)
            .execute(connection)
            .await
    }

    pub async fn save(&self, connection: &mut PgConnection) -> Result<PgQueryResult, sqlx::Error> {
        let name = match &self.name {
            Some(name) => Some(name.as_str()),
            None => None,
        };

        User::update(
            &self.id,
            StoreUserSchema {
                name,
                email: &self.email,
                role_id: self.role_id,
            },
            connection,
        )
        .await
    }

    pub async fn delete(
        id: &Uuid,
        connection: &mut PgConnection,
    ) -> Result<PgQueryResult, sqlx::Error> {
        User::destroy(id, connection).await
    }
}

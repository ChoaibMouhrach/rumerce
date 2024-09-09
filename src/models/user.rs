use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::prelude::FromRow;
use uuid::Uuid;

use super::role::Role;

#[derive(Serialize, FromRow, sqlx::Type)]
pub struct User {
    pub id: Uuid,
    pub name: Option<String>,
    pub email: String,
    pub role_id: Uuid,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, FromRow)]
pub struct PopulatedUser {
    pub user: User,
    pub role: Role,
}

use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Serialize, FromRow, sqlx::Type, Clone)]
pub struct Role {
    pub id: Uuid,
    pub name: String,
    pub created_at: NaiveDateTime,
}

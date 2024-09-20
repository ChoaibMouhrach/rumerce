use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(FromRow, Serialize, sqlx::Type, Debug, Clone)]
pub struct Unit {
    pub id: Uuid,
    pub name: String,
    pub created_at: NaiveDateTime,
}

use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(FromRow, Serialize)]
pub struct Warehouse {
    pub id: Uuid,
    pub name: String,
    pub created_at: NaiveDateTime,
}

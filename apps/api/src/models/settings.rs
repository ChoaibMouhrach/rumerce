use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::prelude::{FromRow, Type};
use uuid::Uuid;

#[derive(Serialize, FromRow, Type)]
#[sqlx(type_name = "settings")]
pub struct Settings {
    pub id: Uuid,
    pub key: String,
    pub value: String,
    pub created_at: NaiveDateTime,
}

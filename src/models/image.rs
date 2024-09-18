use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::prelude::{FromRow, Type};
use uuid::Uuid;

#[derive(FromRow, Debug, Serialize, Type)]
pub struct Image {
    pub id: Uuid,
    pub name: String,
    pub src: String,
    pub created_at: NaiveDateTime,
}

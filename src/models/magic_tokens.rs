use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::prelude::FromRow;
use uuid::Uuid;

use super::user::User;

#[derive(Serialize, FromRow, sqlx::Type, Clone)]
pub struct MagicToken {
    pub id: Uuid,
    pub token: Uuid,
    pub user_id: Uuid,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, FromRow, Clone)]
pub struct PopulatedMagicToken {
    pub user: User,
    pub token: MagicToken,
}

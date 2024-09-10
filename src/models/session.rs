use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::prelude::FromRow;
use uuid::Uuid;

use super::{role::Role, user::User};

#[derive(Serialize, FromRow, sqlx::Type, Clone)]
pub struct Session {
    pub id: Uuid,
    pub session: Uuid,
    pub user_id: Uuid,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, FromRow, Clone)]
pub struct PopulatedSession {
    pub session: Session,
    pub user: User,
    pub role: Role,
}

use chrono::NaiveDateTime;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct StoreMagicTokenSchema {
    pub token: Uuid,
    pub user_id: Uuid,
    pub expires_at: NaiveDateTime,
}

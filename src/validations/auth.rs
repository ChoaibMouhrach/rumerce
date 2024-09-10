use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct SignInSchema {
    pub email: String,
}

#[derive(Deserialize)]
pub struct StoreSessionSchema {
    pub session: Uuid,
    pub user_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignInTokenSchema {
    pub user_id: Uuid,
    pub exp: i64,
}

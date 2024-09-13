use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct SignInSchema {
    #[validate(email)]
    pub email: String,
}

#[derive(Deserialize, Validate)]
pub struct StoreSessionSchema {
    pub session: Uuid,
    pub user_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignInTokenSchema {
    pub user_id: Uuid,
    pub exp: i64,
}

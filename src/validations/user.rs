use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct StoreUserSchema {
    #[validate(length(min = 1))]
    pub name: Option<String>,
    #[validate(email)]
    pub email: String,
    pub role_id: Uuid,
}

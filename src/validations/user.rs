use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct StoreUserSchema {
    pub name: Option<String>,
    pub email: String,
    pub role_id: Uuid,
}

#[derive(Deserialize)]
pub struct UpdateUserSchema {
    pub name: Option<String>,
    pub email: String,
    pub role_id: Uuid,
}

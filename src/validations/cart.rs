use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize)]
pub struct StoreCartSchema {
    pub user_id: Option<Uuid>,
}

#[derive(Deserialize, Validate)]
pub struct StoreCartItemSchema {
    pub variant_id: Uuid,
    #[validate(range(min = 1))]
    pub quantity: i32,
}

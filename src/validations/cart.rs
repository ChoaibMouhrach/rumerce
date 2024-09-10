use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct StoreCartSchema {
    pub user_id: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct StoreCartItemSchema {
    pub variant_id: Uuid,
    pub quantity: i32,
}

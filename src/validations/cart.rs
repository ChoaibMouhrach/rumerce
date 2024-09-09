use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct StoreCartSchema {
    pub user_id: Option<Uuid>,
}

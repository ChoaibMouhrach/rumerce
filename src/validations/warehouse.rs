use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct StoreWarehouseSchema {
    #[validate(length(min = 1))]
    pub name: String,
}

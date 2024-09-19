use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Serialize, Validate)]
pub struct VariantOption {
    #[validate(length(min = 1))]
    pub key: String,
    #[validate(length(min = 1))]
    pub value: String,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct Variant {
    #[validate(length(min = 1))]
    pub options: Vec<VariantOption>,
    #[validate(range(min = 1.0))]
    pub price: f32,
}

#[derive(Deserialize, Validate)]
pub struct StoreProductSchema {
    #[validate(length(min = 1))]
    pub name: String,
    pub description: Option<String>,
    pub unit_id: Uuid,
    pub category_id: Uuid,
    #[validate(length(min = 1))]
    pub variants: Vec<Variant>,
    #[validate(length(min = 1))]
    pub images: Vec<Uuid>,
}

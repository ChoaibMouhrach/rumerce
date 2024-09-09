use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct VariantOption {
    pub key: String,
    pub value: String,
}

#[derive(Deserialize)]
pub struct Variant {
    pub options: Vec<VariantOption>,
    pub price: f32,
}

#[derive(Deserialize)]
pub struct StoreProductSchema {
    pub name: String,
    pub description: Option<String>,
    pub unit_id: Uuid,
    pub category_id: Uuid,
    pub variants: Vec<Variant>,
}

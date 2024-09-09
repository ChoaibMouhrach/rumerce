use serde::Deserialize;

#[derive(Deserialize)]
pub struct StoreCategorySchema {
    pub name: String,
}

#[derive(Deserialize)]
pub struct UpdateCategorySchema {
    pub name: String,
}

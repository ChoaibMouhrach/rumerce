use crate::{
    models::category::Category,
    utils::db::DB,
    validations::category::{StoreCategorySchema, UpdateCategorySchema},
};
use uuid::Uuid;

pub async fn all(db: &DB) -> Vec<Category> {
    sqlx::query_as!(Category, "SELECT * FROM categories")
        .fetch_all(db)
        .await
        .unwrap()
}

pub async fn find(id: &Uuid, db: &DB) -> Option<Category> {
    sqlx::query_as!(Category, "SELECT * FROM categories WHERE id = $1", id)
        .fetch_optional(db)
        .await
        .unwrap()
}

pub async fn insert(input: &StoreCategorySchema, db: &DB) {
    sqlx::query!("INSERT INTO categories(name) VALUES ($1)", input.name)
        .execute(db)
        .await
        .unwrap();
}

pub async fn update(id: &Uuid, input: &UpdateCategorySchema, db: &DB) {
    sqlx::query!(
        "UPDATE categories SET name = $1 WHERE id = $2",
        input.name,
        id
    )
    .execute(db)
    .await
    .unwrap();
}

pub async fn destroy(id: &Uuid, db: &DB) {
    sqlx::query!("DELETE FROM categories WHERE id = $1", id)
        .execute(db)
        .await
        .unwrap();
}

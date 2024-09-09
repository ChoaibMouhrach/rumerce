use uuid::Uuid;

use crate::{
    models::{
        category::Category,
        product::{PopulatedProduct, Product},
        unit::Unit,
    },
    utils::db::DB,
    validations::product::StoreProductSchema,
};

pub async fn all(db: &DB) -> Vec<PopulatedProduct> {
    sqlx::query_as!(
        PopulatedProduct,
        r#"
          SELECT 
            (products.id, products.name, products.description, products.category_id, products.unit_id, products.created_at) AS "product!: Product",
            (units.id, units.name, units.created_at) AS "unit!: Unit",
            (categories.id, categories.name, categories.created_at) AS "category!: Category"
          FROM products
          JOIN units ON products.unit_id = units.id
          JOIN categories ON products.category_id = categories.id
    "#
    ).fetch_all(db).await.unwrap()
}

pub async fn find(id: &Uuid, db: &DB) -> Option<PopulatedProduct> {
    sqlx::query_as!(
        PopulatedProduct,
        r#"
          SELECT 
            (products.id, products.name, products.description, products.category_id, products.unit_id, products.created_at) AS "product!: Product",
            (units.id, units.name, units.created_at) AS "unit!: Unit",
            (categories.id, categories.name, categories.created_at) AS "category!: Category"
          FROM products
          JOIN units ON products.unit_id = units.id
          JOIN categories ON products.category_id = categories.id
          WHERE products.id = $1
        "#,
        id
    ).fetch_optional(db).await.unwrap()
}

pub async fn insert(input: &StoreProductSchema, db: &DB) -> Product {
    sqlx::query_as!(
        Product,
        r#"
          INSERT INTO 
          products(
            name,
            description,
            category_id,
            unit_id
          ) 
          VALUES (
            $1,
            $2,
            $3,
            $4
          )
          RETURNING products.*
        "#,
        input.name,
        input.description,
        input.category_id,
        input.unit_id
    )
    .fetch_one(db)
    .await
    .unwrap()
}

pub async fn update(id: &Uuid, input: &StoreProductSchema, db: &DB) {
    sqlx::query!(
        r#"
          UPDATE products SET name = $2, description = $3, category_id = $4,  unit_id = $5 WHERE id = $1
        "#,
        id,
        input.name,
        input.description,
        input.category_id,
        input.unit_id,
    )
    .execute(db)
    .await
    .unwrap();
}

pub async fn delete(id: &Uuid, db: &DB) {
    sqlx::query!(
        r#"
        DELETE FROM products WHERE id = $1
        "#,
        id,
    )
    .execute(db)
    .await
    .unwrap();
}

use std::collections::HashSet;

use sqlx::postgres::PgQueryResult;
use uuid::Uuid;

use crate::{
    models::{
        category::Category,
        product::{
            PopulatedProduct, Product, ProductVariant, ProductVariantCollection,
            ProductVariantCollectionKey, ProductVariantCollectionValue, SCollection, SProduct,
            SVariant,
        },
        unit::Unit,
    },
    utils::db::DB,
    validations::product::StoreProductSchema,
};

pub async fn all(db: &DB) -> Result<Vec<SProduct>, sqlx::Error> {
    let raw_products = sqlx::query_as!(
        PopulatedProduct,
        r#"
          SELECT 
            (products.id, products.name, products.description, products.category_id, products.unit_id, products.created_at) AS "product!: Product",
            (units.id, units.name, units.created_at) AS "unit!: Unit",
            (categories.id, categories.name, categories.created_at) AS "category!: Category",
            (product_variants.id, product_variants.price, product_variants.product_id) AS "variant!: ProductVariant",
            (product_variant_collections.id, product_variant_collections.variant_id, product_variant_collections.key_id, product_variant_collections.value_id) AS "collection!: ProductVariantCollection",
            (product_variant_collection_keys.id, product_variant_collection_keys.name, product_variant_collection_keys.product_id, product_variant_collections.value_id) AS "key!: ProductVariantCollectionKey",
            (product_variant_collection_values.id, product_variant_collection_values.name, product_variant_collection_values.key_id) AS "value!: ProductVariantCollectionValue"
          FROM products
          JOIN units ON products.unit_id = units.id
          JOIN categories ON products.category_id = categories.id
          JOIN product_variants ON products.id = product_variants.product_id
          JOIN product_variant_collections ON product_variants.id = product_variant_collections.variant_id
          JOIN product_variant_collection_keys ON product_variant_collections.key_id = product_variant_collection_keys.id
          JOIN product_variant_collection_values ON product_variant_collections.value_id = product_variant_collection_values.id
        "#
    ).fetch_all(db).await?;

    // Create a HashSet to remove duplicates by id
    let unique_ids: HashSet<Uuid> = raw_products.iter().map(|s| s.product.id).collect();

    // Create a new vector containing only the unique structs
    let unique_products: Vec<_> = unique_ids
        .into_iter()
        .filter_map(|id| raw_products.iter().find(|s| s.product.id == id))
        .collect();

    let results: Vec<SProduct> = unique_products
        .into_iter()
        .map(|product| {
            // Create a HashSet to remove duplicates by id
            let unique_ids: HashSet<Uuid> = raw_products.iter().map(|s| s.variant.id).collect();

            // Create a new vector containing only the unique structs
            let unique_products: Vec<_> = unique_ids
                .into_iter()
                .filter_map(|id| raw_products.iter().find(|s| s.variant.id == id))
                .collect();

            let variants: Vec<SVariant> = unique_products
                .iter()
                .filter(|p| p.product.id == product.product.id)
                .map(|product| {
                    let collections: Vec<SCollection> = raw_products
                        .iter()
                        .filter(|p| p.variant.id == product.variant.id)
                        .map(|product| SCollection {
                            collection: product.collection.clone(),
                            key: product.key.clone(),
                            value: product.value.clone(),
                        })
                        .collect();

                    SVariant {
                        variant: product.variant.clone(),
                        collections,
                    }
                })
                .collect();

            SProduct {
                product: product.product.clone(),
                category: product.category.clone(),
                unit: product.unit.clone(),
                variants,
            }
        })
        .collect();

    return Ok(results);
}

pub async fn find(id: &Uuid, db: &DB) -> Result<Option<SProduct>, sqlx::Error> {
    let raw_products = sqlx::query_as!(
        PopulatedProduct,
        r#"
          SELECT 
            (products.id, products.name, products.description, products.category_id, products.unit_id, products.created_at) AS "product!: Product",
            (units.id, units.name, units.created_at) AS "unit!: Unit",
            (categories.id, categories.name, categories.created_at) AS "category!: Category",
            (product_variants.id, product_variants.price, product_variants.product_id) AS "variant!: ProductVariant",
            (product_variant_collections.id, product_variant_collections.variant_id, product_variant_collections.key_id, product_variant_collections.value_id) AS "collection!: ProductVariantCollection",
            (product_variant_collection_keys.id, product_variant_collection_keys.name, product_variant_collection_keys.product_id, product_variant_collections.value_id) AS "key!: ProductVariantCollectionKey",
            (product_variant_collection_values.id, product_variant_collection_values.name, product_variant_collection_values.key_id) AS "value!: ProductVariantCollectionValue"
          FROM products
          JOIN units ON products.unit_id = units.id
          JOIN categories ON products.category_id = categories.id
          JOIN product_variants ON products.id = product_variants.product_id
          JOIN product_variant_collections ON product_variants.id = product_variant_collections.variant_id
          JOIN product_variant_collection_keys ON product_variant_collections.key_id = product_variant_collection_keys.id
          JOIN product_variant_collection_values ON product_variant_collections.value_id = product_variant_collection_values.id
          WHERE products.id = $1
        "#,
        id
    ).fetch_all(db).await?;

    // Create a HashSet to remove duplicates by id
    let unique_ids: HashSet<Uuid> = raw_products.iter().map(|s| s.product.id).collect();

    // Create a new vector containing only the unique structs
    let unique_products: Vec<_> = unique_ids
        .into_iter()
        .filter_map(|id| raw_products.iter().find(|s| s.product.id == id))
        .collect();

    let results: Vec<SProduct> = unique_products
        .into_iter()
        .map(|product| {
            // Create a HashSet to remove duplicates by id
            let unique_ids: HashSet<Uuid> = raw_products.iter().map(|s| s.variant.id).collect();

            // Create a new vector containing only the unique structs
            let unique_products: Vec<_> = unique_ids
                .into_iter()
                .filter_map(|id| raw_products.iter().find(|s| s.variant.id == id))
                .collect();

            let variants: Vec<SVariant> = unique_products
                .iter()
                .filter(|p| p.product.id == product.product.id)
                .map(|product| {
                    let collections: Vec<SCollection> = raw_products
                        .iter()
                        .filter(|p| p.variant.id == product.variant.id)
                        .map(|product| SCollection {
                            collection: product.collection.clone(),
                            key: product.key.clone(),
                            value: product.value.clone(),
                        })
                        .collect();

                    SVariant {
                        variant: product.variant.clone(),
                        collections,
                    }
                })
                .collect();

            SProduct {
                product: product.product.clone(),
                category: product.category.clone(),
                unit: product.unit.clone(),
                variants,
            }
        })
        .collect();

    match results.get(0) {
        Some(product) => Ok(Some(product.clone())),
        None => Ok(None),
    }
}

pub async fn insert(input: &StoreProductSchema, db: &DB) -> Result<Product, sqlx::Error> {
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
}

pub async fn update(
    id: &Uuid,
    input: &StoreProductSchema,
    db: &DB,
) -> Result<PgQueryResult, sqlx::Error> {
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
}

pub async fn delete(id: &Uuid, db: &DB) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE FROM products WHERE id = $1
        "#,
        id,
    )
    .execute(db)
    .await
}

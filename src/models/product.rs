use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::{utils::db::DB, validations::product::Variant};

use super::{category::Category, unit::Unit};

#[derive(Debug, Clone, FromRow, Serialize, sqlx::Type)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub unit_id: Uuid,
    pub category_id: Uuid,
    pub created_at: NaiveDateTime,
}

impl Product {
    pub async fn attach_variants(
        &self,
        input_variants: &Vec<Variant>,
        db: &DB,
    ) -> Result<(), sqlx::Error> {
        // prepare keys
        let mut raw_keys: Vec<String> = Vec::new();
        input_variants.iter().for_each(|variant| {
            variant.options.iter().for_each(|option| {
                if raw_keys.contains(&option.key) {
                    return;
                }

                raw_keys.push(option.key.clone())
            })
        });

        // create keys
        let keys = sqlx::query_as!(
            ProductVariantCollectionKey,
            "INSERT INTO product_variant_collection_keys(name, product_id) SELECT * FROM UNNEST($1::TEXT[], $2::UUID[]) RETURNING *",
             &raw_keys,
             &raw_keys.iter().map(|_| {
                self.id
             }).collect::<Vec<Uuid>>(),
            )
    .fetch_all(db)
    .await?;

        // prepare values
        let mut raw_values: Vec<(Uuid, String)> = Vec::new();
        input_variants.iter().for_each(|variant| {
            variant.options.iter().for_each(|option| {
                if let Some(_) = raw_values.iter().find(|(_, value)| value == &option.value) {
                    return;
                }

                let key = keys.iter().find(|key| key.name == option.key).unwrap();
                raw_values.push((key.id, option.value.clone()))
            })
        });

        // create values
        let values = sqlx::query_as!(
        ProductVariantCollectionValue,
        "INSERT INTO product_variant_collection_values(name,key_id) SELECT * FROM UNNEST($1::TEXT[], $2::UUID[]) RETURNING *",
        &raw_values.iter().map(|(_, value)| value.clone()).collect::<Vec<String>>(),
        &raw_values.iter().map(|(key_id, _)| key_id.clone()).collect::<Vec<Uuid>>(),
    )
    .fetch_all(db)
    .await?;

        // prepare variants
        let mut prices: Vec<f32> = Vec::new();
        input_variants.iter().for_each(|variant| {
            prices.push(variant.price);
        });

        // create variants
        let variants = sqlx::query_as!(
        ProductVariant,
        "INSERT INTO product_variants(price, product_id) SELECT * FROM UNNEST($1::REAL[], $2::UUID[]) RETURNING *",
        &prices,
        &prices.iter().map(|_| {
            self.id
        }).collect::<Vec<Uuid>>()
    )
    .fetch_all(db)
    .await?;

        // prepare collections variants
        let mut variants_ids: Vec<Uuid> = Vec::new();
        input_variants
            .iter()
            .enumerate()
            .for_each(|(index, variant)| {
                variant.options.iter().for_each(|_| {
                    variants_ids.push(variants[index].id);
                })
            });

        // prepare collections keys
        let mut keys_ids: Vec<Uuid> = Vec::new();
        input_variants.iter().for_each(|variant| {
            variant.options.iter().for_each(|option| {
                let key = match keys.iter().find(|key| key.name == option.key) {
                    Some(key) => key,
                    None => panic!("Not found"),
                };

                keys_ids.push(key.id)
            })
        });

        // prepare collections values
        let mut values_ids: Vec<Uuid> = Vec::new();
        input_variants.iter().for_each(|variant| {
            variant.options.iter().for_each(|option| {
                let value = match values.iter().find(|value| value.name == option.value) {
                    Some(value) => value,
                    None => panic!("Not found"),
                };

                values_ids.push(value.id)
            })
        });

        // create collections
        sqlx::query_as!(
        ProductVariantCollection,
        "INSERT INTO product_variant_collections(variant_id, key_id, value_id) SELECT * FROM UNNEST($1::UUID[], $2::UUID[], $3::UUID[]) RETURNING *",
        &variants_ids,
        &keys_ids,
        &values_ids
    )
    .fetch_all(db)
    .await?;

        Ok(())
    }

    pub async fn detach_variants(&self, db: &DB) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "DELETE FROM product_variants WHERE product_id = $1",
            self.id
        )
        .execute(db)
        .await?;

        sqlx::query!(
            "DELETE FROM product_variant_collection_keys WHERE product_id = $1",
            self.id
        )
        .execute(db)
        .await?;

        Ok(())
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct PopulatedProduct {
    pub product: Product,
    pub category: Category,
    pub unit: Unit,
    pub variant: ProductVariant,
    pub collection: ProductVariantCollection,
    pub key: ProductVariantCollectionKey,
    pub value: ProductVariantCollectionValue,
}

#[derive(Debug, Clone, FromRow, Serialize, sqlx::Type)]
pub struct ProductVariantCollectionKey {
    pub id: Uuid,
    pub name: String,
    pub product_id: Uuid,
}

#[derive(Debug, Clone, FromRow, Serialize, sqlx::Type)]
pub struct ProductVariantCollectionValue {
    pub id: Uuid,
    pub name: String,
    pub key_id: Uuid,
}

#[derive(Debug, Clone, FromRow, Serialize, sqlx::Type)]
pub struct ProductVariant {
    pub id: Uuid,
    pub price: f32,
    pub product_id: Uuid,
}

#[derive(Debug, Clone, FromRow, Serialize, sqlx::Type)]
pub struct ProductVariantCollection {
    pub id: Uuid,
    pub variant_id: Uuid,
    pub key_id: Uuid,
    pub value_id: Uuid,
}

#[derive(Serialize, Debug, Clone)]
pub struct SCollection {
    pub collection: ProductVariantCollection,
    pub key: ProductVariantCollectionKey,
    pub value: ProductVariantCollectionValue,
}

#[derive(Serialize, Debug, Clone)]
pub struct SVariant {
    pub variant: ProductVariant,
    pub collections: Vec<SCollection>,
}

#[derive(Serialize, Debug, Clone)]
pub struct SProduct {
    pub product: Product,
    pub category: Category,
    pub unit: Unit,
    pub variants: Vec<SVariant>,
}

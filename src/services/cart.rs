use sqlx::{postgres::PgQueryResult, PgConnection};
use uuid::Uuid;

use crate::{
    models::cart::{Cart, CartItem, PopulatedCart},
    validations::cart::{StoreCartItemSchema, StoreCartSchema},
};

pub async fn all(db: &mut PgConnection) -> Result<Vec<PopulatedCart>, sqlx::Error> {
    let carts = sqlx::query_as!(
        PopulatedCart,
        r#"
            SELECT 
                (carts.id, carts.user_id) as "cart!: Cart",
                array_agg((cart_items.id, cart_items.quantity, cart_items.cart_id, cart_items.variant_id)) as "items!: Vec<CartItem>"
            FROM carts
            LEFT JOIN cart_items ON carts.id = cart_items.cart_id
            GROUP BY carts.id, "cart!: Cart"
        "#)
        .fetch_all(&mut *db)
        .await?;

    Ok(carts
        .into_iter()
        .map(|cart| PopulatedCart {
            cart: cart.cart,
            items: cart
                .items
                .into_iter()
                .filter(|item| item.id.is_some())
                .collect::<Vec<CartItem>>(),
        })
        .collect::<Vec<PopulatedCart>>())
}

pub async fn find(id: &Uuid, db: &mut PgConnection) -> Result<Option<PopulatedCart>, sqlx::Error> {
    let cart = sqlx::query_as!(
        PopulatedCart,
        r#"
            SELECT 
                (carts.id, carts.user_id) as "cart!: Cart",
                array_agg((cart_items.id, cart_items.quantity, cart_items.cart_id, cart_items.variant_id)) as "items!: Vec<CartItem>"
            FROM carts
            LEFT JOIN cart_items ON carts.id = cart_items.cart_id
            WHERE carts.id = $1
            GROUP BY carts.id, "cart!: Cart"
        "#,
        id
    )
    .fetch_optional(&mut *db)
    .await?;

    match cart {
        Some(cart) => Ok(Some(PopulatedCart {
            cart: cart.cart,
            items: cart
                .items
                .into_iter()
                .filter(|item| item.id.is_some())
                .collect::<Vec<CartItem>>(),
        })),
        None => Ok(None),
    }
}

pub async fn find_by_user_id(
    user_id: &Uuid,
    db: &mut PgConnection,
) -> Result<Option<PopulatedCart>, sqlx::Error> {
    let cart = sqlx::query_as!(
        PopulatedCart,
        r#"
            SELECT 
                (carts.id, carts.user_id) as "cart!: Cart",
                array_agg((cart_items.id, cart_items.quantity, cart_items.cart_id, cart_items.variant_id)) as "items!: Vec<CartItem>"
            FROM carts
            LEFT JOIN cart_items ON carts.id = cart_items.cart_id
            WHERE carts.user_id = $1
            GROUP BY carts.id, "cart!: Cart"
        "#,
        user_id
    )
    .fetch_optional(&mut *db)
    .await?;

    match cart {
        Some(cart) => Ok(Some(PopulatedCart {
            cart: cart.cart,
            items: cart
                .items
                .into_iter()
                .filter(|item| item.id.is_some())
                .collect::<Vec<CartItem>>(),
        })),
        None => Ok(None),
    }
}

pub async fn insert(
    input: &StoreCartSchema,
    db: &mut PgConnection,
) -> Result<PopulatedCart, sqlx::Error> {
    sqlx::query_as!(
        PopulatedCart,
        r#"
            WITH new_cart AS (
                INSERT INTO carts(user_id) VALUES ($1) RETURNING *
            ) 
            SELECT 
                (new_cart.id, new_cart.user_id) as "cart!: Cart",
                array_agg((cart_items.id, cart_items.quantity, cart_items.cart_id, cart_items.variant_id)) as "items!: Vec<CartItem>"
            FROM new_cart
            LEFT JOIN cart_items ON new_cart.id = cart_items.cart_id
            GROUP BY new_cart.id, "cart!: Cart"
        "#,
        input.user_id
    )
    .fetch_one(&mut *db)
    .await
}

pub async fn update(
    id: &Uuid,
    input: &StoreCartSchema,
    db: &mut PgConnection,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        "UPDATE carts SET user_id = $2 WHERE id = $1",
        id,
        input.user_id
    )
    .execute(&mut *db)
    .await
}

pub async fn destroy(id: &Uuid, db: &mut PgConnection) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!("DELETE FROM carts WHERE id = $1", id,)
        .execute(&mut *db)
        .await
}

pub async fn add_item(
    cart_id: &Uuid,
    input: &StoreCartItemSchema,
    connection: &mut PgConnection,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        "INSERT INTO cart_items(variant_id, cart_id, quantity) VALUES ($1, $2, $3)",
        input.variant_id,
        cart_id,
        input.quantity
    )
    .execute(connection)
    .await
}

pub async fn delete_item(
    cart_id: &Uuid,
    item_id: &Uuid,
    connection: &mut PgConnection,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        "DELETE FROM cart_items WHERE cart_id = $1 AND id = $2",
        cart_id,
        item_id
    )
    .execute(connection)
    .await
}

pub async fn update_item(
    cart_id: &Uuid,
    item_id: &Uuid,
    input: &StoreCartItemSchema,
    connection: &mut PgConnection,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        "UPDATE cart_items SET variant_id = $3, cart_id = $4, quantity = $5 WHERE cart_id = $1 AND id = $2",
        cart_id,
        item_id,
        input.variant_id,
        cart_id,
        input.quantity
    )
    .execute(connection)
    .await
}

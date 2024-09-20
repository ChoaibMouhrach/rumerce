use serde::Serialize;
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(FromRow, Serialize, sqlx::Type)]
#[sqlx(type_name = "carts")]
pub struct Cart {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
}

#[derive(Serialize)]
pub struct PopulatedCart {
    pub cart: Cart,
    pub items: Vec<CartItem>,
}

#[derive(FromRow, Serialize, sqlx::Type)]
#[sqlx(type_name = "cart_items")]
pub struct CartItem {
    pub id: Option<Uuid>,
    pub quantity: Option<i32>,
    pub cart_id: Option<Uuid>,
    pub variant_id: Option<Uuid>,
}

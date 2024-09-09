use crate::{
    models::unit::Unit,
    utils::db::DB,
    validations::unit::{StoreUnitSchema, UpdateUnitSchema},
};
use uuid::Uuid;

pub async fn all(db: &DB) -> Vec<Unit> {
    sqlx::query_as!(Unit, "SELECT * FROM units")
        .fetch_all(db)
        .await
        .unwrap()
}

pub async fn find(id: &Uuid, db: &DB) -> Option<Unit> {
    sqlx::query_as!(Unit, "SELECT * FROM units WHERE id = $1", id)
        .fetch_optional(db)
        .await
        .unwrap()
}

pub async fn insert(input: &StoreUnitSchema, db: &DB) {
    sqlx::query!("INSERT INTO units(name) VALUES ($1)", input.name)
        .execute(db)
        .await
        .unwrap();
}

pub async fn update(id: &Uuid, input: &UpdateUnitSchema, db: &DB) {
    sqlx::query!("UPDATE units SET name = $1 WHERE id = $2", input.name, id)
        .execute(db)
        .await
        .unwrap();
}

pub async fn destroy(id: &Uuid, db: &DB) {
    sqlx::query!("DELETE FROM units WHERE id = $1", id)
        .execute(db)
        .await
        .unwrap();
}

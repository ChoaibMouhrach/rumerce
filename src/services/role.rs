use crate::{
    models::role::Role,
    utils::db::DB,
    validations::role::{StoreRoleSchema, UpdateRoleSchema},
};
use uuid::Uuid;

pub async fn all(db: &DB) -> Vec<Role> {
    sqlx::query_as!(Role, "SELECT * FROM roles")
        .fetch_all(db)
        .await
        .unwrap()
}

pub async fn find(id: &Uuid, db: &DB) -> Option<Role> {
    sqlx::query_as!(Role, "SELECT * FROM roles WHERE id = $1", id)
        .fetch_optional(db)
        .await
        .unwrap()
}

pub async fn insert(input: &StoreRoleSchema, db: &DB) {
    sqlx::query!("INSERT INTO roles(name) VALUES ($1)", input.name)
        .execute(db)
        .await
        .unwrap();
}

pub async fn update(id: &Uuid, input: &UpdateRoleSchema, db: &DB) {
    sqlx::query!("UPDATE roles SET name = $1 WHERE id = $2", input.name, id)
        .execute(db)
        .await
        .unwrap();
}

pub async fn destroy(id: &Uuid, db: &DB) {
    sqlx::query!("DELETE FROM roles WHERE id = $1", id)
        .execute(db)
        .await
        .unwrap();
}

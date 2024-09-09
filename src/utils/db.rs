use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub type DB = Pool<Postgres>;

pub async fn init(url: &str) -> DB {
    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(url)
        .await
        .unwrap();
    db
}

use dotenvy::dotenv;
use std::env;

pub struct Env {
    // APP
    pub app_url: String,

    // DATABASE
    pub database_url: String,
}

pub fn init() -> Env {
    dotenv().expect(".env file not found");

    Env {
        // APP
        app_url: env::var("APP_URL").unwrap(),

        // DATABASE
        database_url: env::var("DATABASE_URL").unwrap(),
    }
}

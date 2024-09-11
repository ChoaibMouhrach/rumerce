use dotenvy::dotenv;
use std::env;

#[derive(Clone)]
pub struct Env {
    // APP
    pub app_url: String,
    pub app_secret: String,

    // CLIENT
    pub client_url: String,

    // RESEND
    pub resend_token: String,
    pub resend_domain: String,

    // DATABASE
    pub database_url: String,
}

pub fn init() -> Env {
    dotenv().expect(".env file not found");

    Env {
        // APP
        app_url: env::var("APP_URL").unwrap(),
        app_secret: env::var("APP_SECRET").unwrap(),

        // CLIENT
        client_url: env::var("CLIENT_URL").unwrap(),

        // RESEND
        resend_token: env::var("RESEND_TOKEN").unwrap(),
        resend_domain: env::var("RESEND_DOMAIN").unwrap(),

        // DATABASE
        database_url: env::var("DATABASE_URL").unwrap(),
    }
}

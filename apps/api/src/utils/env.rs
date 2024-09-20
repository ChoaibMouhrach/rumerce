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

fn dot_env(name: &str) -> String {
    env::var(name).expect(&format!("{} is missing", name))
}

pub fn init() -> Env {
    dotenv().expect(".env file not found");

    Env {
        // APP
        app_url: dot_env("APP_URL"),
        app_secret: dot_env("APP_SECRET"),

        // CLIENT
        client_url: dot_env("CLIENT_URL"),

        // RESEND
        resend_token: dot_env("RESEND_TOKEN"),
        resend_domain: dot_env("RESEND_DOMAIN"),

        // DATABASE
        database_url: dot_env("DATABASE_URL"),
    }
}

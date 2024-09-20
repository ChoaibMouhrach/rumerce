#![allow(dead_code)]
use axum::{async_trait, Router};
use resend_rs::types::CreateEmailBaseOptions;
use rumerce::{
    create_app,
    models::{role::Role, session::Session, user::PopulatedUser},
    services,
    utils::{constants::ROLES, db, env, mailer::Mail},
    validations::{auth::StoreSessionSchema, role::StoreRoleSchema, user::StoreUserSchema},
    State,
};
use sqlx::pool::PoolConnection;
use testcontainers::ContainerAsync;
use testcontainers_modules::postgres::Postgres;
use uuid::Uuid;

#[derive(Clone)]
pub struct Mailer;

#[async_trait]
impl Mail for Mailer {
    async fn mail(&self, _: CreateEmailBaseOptions) -> Result<(), resend_rs::Error> {
        Ok(())
    }
}

pub struct Config {
    pub app: Router,
    pub connection: PoolConnection<sqlx::Postgres>,
    pub member: Role,
    pub admin: Role,
}

pub async fn init(container: &ContainerAsync<Postgres>) -> Config {
    let db = db::init(&format!(
        "postgresql://postgres:postgres@localhost:{}/postgres",
        container.get_host_port_ipv4(5432).await.unwrap()
    ))
    .await;

    sqlx::migrate!("./migrations").run(&db).await.unwrap();

    let env = env::init();
    let mailer = Box::new(Mailer);

    let state = State { db, env, mailer };

    let mut connection = state.db.acquire().await.unwrap();

    let member = services::role::insert(&StoreRoleSchema { name: ROLES.member }, &mut connection)
        .await
        .unwrap();

    let admin = services::role::insert(&StoreRoleSchema { name: ROLES.admin }, &mut connection)
        .await
        .unwrap();

    let app = create_app(state.clone());

    Config {
        app,
        connection,
        member,
        admin,
    }
}

pub async fn auth(
    role: &Role,
    connection: &mut PoolConnection<sqlx::Postgres>,
) -> (PopulatedUser, Session) {
    let user = services::user::insert(
        &StoreUserSchema {
            name: None,
            email: "example@example.com".to_string(),
            role_id: role.id.clone(),
        },
        connection,
    )
    .await
    .unwrap();

    let session = services::sessions::insert(
        StoreSessionSchema {
            session: Uuid::new_v4(),
            user_id: user.user.id,
        },
        connection,
    )
    .await
    .unwrap();

    (user, session)
}

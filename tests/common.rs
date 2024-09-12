use axum::{async_trait, Router};
use resend_rs::types::CreateEmailBaseOptions;
use rumerce::{
    create_app, services,
    utils::{constants::ROLES, db, env, mailer::Mail},
    validations::role::StoreRoleSchema,
    State,
};
use sqlx::pool::PoolConnection;
use testcontainers::ContainerAsync;
use testcontainers_modules::postgres::Postgres;

#[derive(Clone)]
pub struct Mailer;

#[async_trait]
impl Mail for Mailer {
    async fn mail(&self, _: CreateEmailBaseOptions) -> Result<(), resend_rs::Error> {
        Ok(())
    }
}

pub async fn init(
    container: &ContainerAsync<Postgres>,
) -> (Router, PoolConnection<sqlx::Postgres>) {
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

    services::role::insert(
        &StoreRoleSchema {
            name: ROLES.member.to_string(),
        },
        &mut connection,
    )
    .await
    .unwrap();

    services::role::insert(
        &StoreRoleSchema {
            name: ROLES.admin.to_string(),
        },
        &mut connection,
    )
    .await
    .unwrap();

    let app = create_app(state.clone());

    (app, connection)
}

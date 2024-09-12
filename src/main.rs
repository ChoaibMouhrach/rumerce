use resend_rs::Resend;
use rumerce::{
    create_app,
    utils::{db, env, mailer::Mailer},
    State,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let env = env::init();
    let db = db::init(&env.database_url).await;
    let mailer = Box::new(Mailer {
        client: Resend::new(&env.resend_token),
    });

    let state = State { env, db, mailer };

    let app = create_app(state.clone());

    println!("Server running on {:#?}", &state.env.app_url);
    let listener = tokio::net::TcpListener::bind(&state.env.app_url)
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

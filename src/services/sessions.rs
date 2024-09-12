use sqlx::{postgres::PgQueryResult, PgConnection};
use uuid::Uuid;

use crate::{
    models::session::{PopulatedSession, Session},
    models::{role::Role, user::User},
    validations::auth::StoreSessionSchema,
};

pub async fn find(
    id: &Uuid,
    db: &mut PgConnection,
) -> Result<Option<PopulatedSession>, sqlx::Error> {
    sqlx::query_as!(
        PopulatedSession,
        r#"
            SELECT 
                (sessions.id, sessions.session, sessions.user_id, sessions.created_at) as "session!: Session" ,
                (users.id, users.name, users.email, users.role_id, users.created_at) as "user!: User",
                (roles.id, roles.name, roles.created_at) as "role!: Role"
            FROM sessions
            JOIN users ON users.id = sessions.user_id
            JOIN roles ON users.role_id = roles.id
            WHERE sessions.id = $1
        "#,
        id
    ).fetch_optional(db).await
}

pub async fn find_by_session(
    session: &Uuid,
    db: &mut PgConnection,
) -> Result<Option<PopulatedSession>, sqlx::Error> {
    sqlx::query_as!(
        PopulatedSession,
        r#"
            SELECT 
                (sessions.id, sessions.session, sessions.user_id, sessions.created_at) as "session!: Session" ,
                (users.id, users.name, users.email, users.role_id, users.created_at) as "user!: User",
                (roles.id, roles.name, roles.created_at) as "role!: Role"
            FROM sessions
            JOIN users ON users.id = sessions.user_id
            JOIN roles ON users.role_id = roles.id
            WHERE sessions.session = $1
        "#,
        session
    ).fetch_optional(db).await
}

pub async fn find_by_user_id(
    user_id: &Uuid,
    db: &mut PgConnection,
) -> Result<Option<PopulatedSession>, sqlx::Error> {
    sqlx::query_as!(
        PopulatedSession,
        r#"
            SELECT 
                (sessions.id, sessions.session, sessions.user_id, sessions.created_at) as "session!: Session" ,
                (users.id, users.name, users.email, users.role_id, users.created_at) as "user!: User",
                (roles.id, roles.name, roles.created_at) as "role!: Role"
            FROM sessions
            JOIN users ON users.id = sessions.user_id
            JOIN roles ON users.role_id = roles.id
            WHERE sessions.user_id = $1
        "#,
        user_id
    ).fetch_optional(db).await
}

pub async fn insert(
    input: StoreSessionSchema,
    db: &mut PgConnection,
) -> Result<Session, sqlx::Error> {
    sqlx::query_as!(
        Session,
        "INSERT INTO sessions(session, user_id) VALUES ($1, $2) RETURNING *",
        input.session,
        input.user_id
    )
    .fetch_one(db)
    .await
}

pub async fn delete(id: &Uuid, db: &mut PgConnection) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!("DELETE FROM sessions WHERE id = $1", id)
        .execute(db)
        .await
}

pub async fn delete_by_session(
    session: &Uuid,
    db: &mut PgConnection,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!("DELETE FROM sessions WHERE session = $1", session)
        .execute(db)
        .await
}

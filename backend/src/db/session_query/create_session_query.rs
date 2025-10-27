use sqlx::{Error, PgPool};
use crate::models::session_models::session::{NewSession, Session};

pub async fn create_session_query(
    pool: &PgPool,
    new_session: NewSession,
) -> Result<Session, Error> {
    let session = sqlx::query_as::<_, Session>(
        r#"
        INSERT INTO sessions (user_id, session_token, created_at, expires_at, ip_address, user_agent, is_active)
        VALUES ($1, $2, NOW(), $3, $4, $5, TRUE)
        RETURNING id, user_id, session_token, created_at, expires_at, ip_address, user_agent, is_active
        "#
    )
        .bind(new_session.user_id)
        .bind(new_session.session_token)
        .bind(new_session.expires_at)
        .bind(new_session.ip_address)
        .bind(new_session.user_agent)
        .persistent(false)
        .fetch_one(pool)
        .await?;

    Ok(session)
}

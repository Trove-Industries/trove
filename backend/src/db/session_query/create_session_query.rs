use chrono::{Duration, Utc};
use sqlx::{Error, PgPool};
use uuid::Uuid;
use crate::models::session_models::session::{NewSession, Session};


pub async fn create_new_session(pool: &PgPool, new_session: NewSession) -> Result<Session, sqlx::Error> {
    let token = Uuid::new_v4().to_string();
    let now = Utc::now();
    let expires_at = now + Duration::hours(6);

    let row = sqlx::query_as::<_, Session>(
        r#"
        INSERT INTO sessions (user_id, session_token, created_at, expires_at, ip_address, user_agent, is_active)
        VALUES ($1, $2, $3, $4, $5, $6, TRUE)
        RETURNING *
        "#
    )
        .bind(new_session.user_id)
        .bind(token)
        .bind(now)
        .bind(expires_at)
        .bind(new_session.ip_address)
        .bind(new_session.user_agent)
        .fetch_one(pool)
        .await?;

    Ok(row)
}

use sqlx::{PgPool, Row};
use uuid::Uuid;
use chrono::{Duration, Utc};
use anyhow::Result;
use crate::models::session_models::session::Session;

impl Session {
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }
}

/// Create a new session and return the session record
pub async fn create_session_service(
    pool: &PgPool,
    user_id: i32,
    ip_address: Option<String>,
    user_agent: Option<String>,
) -> Result<Session> {
    let session_token = Uuid::new_v4();
    let created_at = Utc::now();
    let expires_at = created_at + Duration::days(10);

    let row = sqlx::query(
        r#"
        INSERT INTO sessions (user_id, session_token, created_at, expires_at, ip_address, user_agent, is_active)
        VALUES ($1, $2, $3, $4, $5, $6, TRUE)
        RETURNING id, user_id, session_token, created_at, expires_at, ip_address, user_agent, is_active
        "#,
    )
        .bind(user_id)
        .bind(&session_token)
        .bind(created_at)
        .bind(expires_at)
        .bind(ip_address)
        .bind(user_agent)
        .persistent(false)
        .fetch_one(pool)
        .await?;

    Ok(Session {
        id: row.get("id"),
        user_id: row.get("user_id"),
        session_token: row.get("session_token"),
        created_at: row.get("created_at"),
        expires_at: row.get("expires_at"),
        ip_address: row.get("ip_address"),
        user_agent: row.get("user_agent"),
        is_active: row.get("is_active"),
    })
}

/// Validate a session from the session_token
pub async fn validate_session(pool: &PgPool, token: &Uuid) -> Result<Option<Session>> {
    let row = sqlx::query(
        "SELECT * FROM sessions WHERE session_token = $1 AND is_active = TRUE LIMIT 1",
    )
        .bind(token)
        .fetch_optional(pool)
        .await?;

    if let Some(r) = row {
        let session = Session {
            id: r.get("id"),
            user_id: r.get("user_id"),
            session_token: r.get("session_token"),
            created_at: r.get("created_at"),
            expires_at: r.get("expires_at"),
            ip_address: r.get("ip_address"),
            user_agent: r.get("user_agent"),
            is_active: r.get("is_active"),
        };
        if session.is_expired() {
            deactivate_session(pool, &session.session_token).await?;
            Ok(None)
        } else {
            Ok(Some(session))
        }
    } else {
        Ok(None)
    }
}

/// Deactivate a session when expired or log out
pub async fn deactivate_session(pool: &PgPool, token: &Uuid) -> Result<()> {
    sqlx::query("UPDATE sessions SET is_active = FALSE WHERE session_token = $1")
        .bind(token)
        .execute(pool)
        .await?;
    Ok(())
}

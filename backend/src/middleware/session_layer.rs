/*
use axum::{
    async_trait,
    extract::{FromRequestParts, State},
    http::{request::Parts, StatusCode},
};
use chrono::Utc;
use sqlx::PgPool;
use tower_cookies::Cookies;
use tracing::error;

use crate::models::session::Session;

pub struct AuthenticatedSession(pub Session);

#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedSession
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let cookies = parts
            .extensions
            .get::<Cookies>()
            .ok_or((StatusCode::UNAUTHORIZED, "No cookies found".to_string()))?;

        let session_token = cookies
            .get("session_token")
            .map(|c| c.value().to_string())
            .ok_or((StatusCode::UNAUTHORIZED, "Missing session token".to_string()))?;

        let pool = parts
            .extensions
            .get::<PgPool>()
            .cloned()
            .ok_or((StatusCode::INTERNAL_SERVER_ERROR, "Database pool not found".to_string()))?;

        let session = match sqlx::query_as::<_, Session>(
            r#"
            SELECT id, user_id, session_token, created_at, expires_at, ip_address, user_agent, is_active
            FROM sessions
            WHERE session_token = $1 AND is_active = TRUE
            "#,
        )
            .bind(&session_token)
            .fetch_optional(&pool)
            .await
        {
            Ok(Some(session)) => session,
            Ok(None) => {
                return Err((StatusCode::UNAUTHORIZED, "Invalid session token".into()));
            }
            Err(e) => {
                error!("❌ DB error while validating session: {:?}", e);
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Database error validating session".into(),
                ));
            }
        };

        if session.expires_at < Utc::now() {
            // Mark session inactive
            let _ = sqlx::query("UPDATE sessions SET is_active = FALSE WHERE id = $1")
                .bind(session.id)
                .execute(&pool)
                .await;

            return Err((StatusCode::UNAUTHORIZED, "Session expired".into()));
        }

        Ok(AuthenticatedSession(session))
    }
}
 */
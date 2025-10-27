/*
use tower_cookies::Cookies;
use axum::{http::StatusCode, extract::State};
use sqlx::PgPool;
use chrono::Utc;

pub async fn validate_session(cookies: Cookies, State(pool): State<PgPool>)
                              -> Result<i32, (StatusCode, String)> {

    if let Some(cookie) = cookies.get("session_token") {
        let token = cookie.value();

        let session = sqlx::query!(
            "SELECT user_id, expires_at, is_active FROM sessions WHERE session_token = $1",
            token
        )
            .fetch_optional(&pool)
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "DB Error".into()))?;

        if let Some(s) = session {
            if s.is_active && s.expires_at > Utc::now() {
                return Ok(s.user_id);
            }
        }
    }
    Err((StatusCode::UNAUTHORIZED, "Invalid or expired session".into()))
}
 */

// src/handlers/session_handler/restore.rs
use axum::{
    extract::State,
    response::IntoResponse,
    Json,
};
use axum_extra::extract::cookie::CookieJar;
use http::StatusCode;
use sqlx::PgPool;
use tracing::{error, info};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RestoreSessionResponse {
    pub session_token: String,
    pub restaurant_name: String,
    pub user_id: i32,
    pub has_categories: bool,
    pub has_meals: bool,
}

pub async fn restore_session_handler(
    State(pool): State<PgPool>,
    jar: CookieJar,
) -> impl IntoResponse {
    // Get session token from cookie
    let session_token = match jar.get("session_token") {
        Some(cookie) => cookie.value().to_string(),
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "error": "No active session found"
                })),
            )
                .into_response();
        }
    };

    info!("🔄 Restoring session: {}", session_token);

    // Get user_id from session
    let user_id: i32 = match sqlx::query_scalar::<_, i32>(
        r#"
        SELECT user_id
        FROM sessions
        WHERE session_token = $1
          AND is_active = TRUE
          AND expires_at > NOW()
        "#
    )
        .bind(&session_token)
        .fetch_one(&pool)
        .await
    {
        Ok(id) => id,
        Err(e) => {
            error!("❌ Invalid or expired session: {:?}", e);
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "error": "Invalid or expired session"
                })),
            )
                .into_response();
        }
    };

    // Get restaurant name
    let restaurant_name: String = match sqlx::query_scalar::<_, String>(
        r#"
        SELECT restaurant_name
        FROM restaurants
        WHERE user_id = $1
        ORDER BY created_at DESC
        LIMIT 1
        "#
    )
        .bind(user_id)
        .fetch_one(&pool)
        .await
    {
        Ok(name) => name,
        Err(e) => {
            error!("❌ No restaurant found for user: {:?}", e);
            return (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "error": "No restaurant found"
                })),
            )
                .into_response();
        }
    };

    // Check if user has categories
    let has_categories: bool = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*)
        FROM categories c
        JOIN restaurants r ON c.restaurant_id = r.id
        WHERE r.user_id = $1
        "#
    )
        .bind(user_id)
        .fetch_one(&pool)
        .await
        .unwrap_or(0) > 0;

    // Check if user has meals
    let has_meals: bool = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*)
        FROM meals m
        JOIN restaurants r ON m.restaurant_id = r.id
        WHERE r.user_id = $1
        "#
    )
        .bind(user_id)
        .fetch_one(&pool)
        .await
        .unwrap_or(0) > 0;

    info!("✅ Session restored for user {}: {}", user_id, restaurant_name);

    (
        StatusCode::OK,
        Json(RestoreSessionResponse {
            session_token,
            restaurant_name,
            user_id,
            has_categories,
            has_meals,
        }),
    )
        .into_response()
}
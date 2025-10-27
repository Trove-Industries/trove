use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use http::HeaderMap;
use sqlx::PgPool;
use serde::Serialize;
use tracing::{info, error};
use crate::admin::admin_service::admin_service::get_all_users_service;
use crate::models::user_models::user::User;
use crate::state::AppState;

#[derive(Serialize)]
struct CleanupResponse {
    marked_inactive: u64,
    deleted: u64,
    message: String,
}

pub async fn get_all_users_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    let pool = state.pool;
    
    let users = get_all_users_service(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(users))
}

async fn cleanup_expired_sessions(pool: &PgPool) -> Result<(u64, u64), sqlx::Error> {
    let now = chrono::Utc::now();

    // 1️⃣ Mark expired sessions inactive
    let inactive_count = sqlx::query!(
        r#"UPDATE sessions SET is_active = FALSE WHERE expires_at < $1 AND is_active = TRUE"#,
        now
    )
        .execute(pool)
        .await?
        .rows_affected();

    // 2️⃣ Delete sessions older than 30 days
    let deleted_count = sqlx::query!(
        r#"DELETE FROM sessions WHERE is_active = FALSE AND expires_at < $1"#,
        now - chrono::Duration::days(30)
    )
        .execute(pool)
        .await?
        .rows_affected();

    Ok((inactive_count, deleted_count))
}
/*
pub async fn admin_cleanup_sessions_handler(
    State(pool): State<PgPool>,
    headers: HeaderMap,
) -> Result<Json<CleanupResponse>, (StatusCode, String)> {
    let expected_key = std::env::var("ADMIN_API_KEY").unwrap_or_default();

    let Some(key_header) = headers.get("x-admin-key") else {
        return Err((StatusCode::UNAUTHORIZED, "Missing admin key".into()));
    };

    if key_header != expected_key {
        return Err((StatusCode::UNAUTHORIZED, "Invalid admin key".into()));
    }

    // Continue to cleanup
    match cleanup_expired_sessions(&pool).await {
        Ok((inactive, deleted)) => Ok(Json(CleanupResponse {
            marked_inactive: inactive,
            deleted,
            message: "Cleanup completed successfully".into(),
        })),
        Err(e) => {
            error!("❌ Cleanup failed: {:?}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to clean sessions".into()))
        }
    }
}

pub async fn admin_cleanup_sessions_handler(
    State(pool): State<PgPool>,
) -> Result<Json<CleanupResponse>, (StatusCode, String)> {
    match cleanup_expired_sessions(&pool).await {
        Ok((inactive_count, deleted_count)) => {
            info!("✅ Manual cleanup successful: {} inactive, {} deleted", inactive_count, deleted_count);
            Ok(Json(CleanupResponse {
                marked_inactive: inactive_count,
                deleted: deleted_count,
                message: "Cleanup completed successfully".to_string(),
            }))
        }
        Err(e) => {
            error!("❌ Manual cleanup failed: {:?}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to clean sessions".to_string()))
        }
    }
}
*/
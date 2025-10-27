use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use http::HeaderMap;
use sqlx::{Execute, PgPool};
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

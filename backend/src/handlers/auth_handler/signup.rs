use axum::extract::State;
use axum::Json;
use axum_extra::extract::CookieJar;
use http::StatusCode;
use crate::models::auth_models::signup::{SignupRequest, SignupResponse};
use crate::services::auth_services::signup::signup_service;
use crate::state::AppState;
use crate::utils::get_id::get_restaurant_id;
use serde_json::json;

pub async fn signup_handler(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(signup_request): Json<SignupRequest>,
) -> Result<Json<SignupResponse>, (StatusCode, Json<serde_json::Value>)> {
    let pool = &state.pool;
    let supabase = &state.supabase;

    let restaurant_id = get_restaurant_id(pool, &jar).await?;

    let signup = signup_service(pool, supabase, signup_request, restaurant_id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": e }))
            )
        })?;

    Ok(Json(signup))
}

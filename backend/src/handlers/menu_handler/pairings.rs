use axum::extract::{Path, State};
use axum::Json;
use axum_extra::extract::CookieJar;
use http::StatusCode;
use serde_json::json;
use sqlx::PgPool;
use tracing::log::error;
use crate::models::menu_models::pairings::{NewPairing, Pairing};
use crate::services::menu_services::pairings::{create_pairings_service, get_pairings_by_session_service, get_pairings_by_subdomain_service};
use crate::state::AppState;
use crate::utils::get_id::get_restaurant_id;

pub async fn create_pairings_handler(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(new_pairing): Json<NewPairing>
)->Result<Json<Pairing>, (StatusCode, Json<serde_json::Value>)>{
    let pool = state.pool;

    let restaurant_id = get_restaurant_id(&pool, &jar).await?;

    let pairing = create_pairings_service(&pool, new_pairing.meal_id, restaurant_id, new_pairing)
        .await
        .map_err(|e| {
            tracing::error!("❌ Failed to create pairing: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Failed to create pairing",
                    "details": e.to_string()
                }))
            )
        })?;
    Ok(Json(pairing))
}

pub async fn get_pairings_by_subdomain_handler(
    State(state): State<AppState>,
    Path(restaurant_name): Path<String>
)->Result<Json<Vec<Pairing>>, (StatusCode, Json<serde_json::Value>)>{
    let pool = state.pool;

    let pairing = get_pairings_by_subdomain_service(&pool, &restaurant_name)
        .await
        .map_err(|e| {
            tracing::error!("❌ Failed to get pairing: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Failed to get pairing",
                    "details": e.to_string()
                }))
            )
        })?;
    Ok(Json(pairing))
}

pub async fn get_pairings_by_session_handler(
    State(state): State<AppState>,
    jar: CookieJar,
)->Result<Json<Vec<Pairing>>, (StatusCode, Json<serde_json::Value>)>{
    let pool = state.pool;

    let restaurant_id = get_restaurant_id(&pool, &jar).await?;

    let pairing = get_pairings_by_session_service(&pool, restaurant_id)
        .await
        .map_err(|e| {
            error!("Failed to get pairing: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                "error": "Failed to get pairing",
                "details": e.to_string()
            }))
            )
        })?;
    Ok(Json(pairing))
}
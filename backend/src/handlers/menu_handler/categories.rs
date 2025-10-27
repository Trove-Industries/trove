use axum::extract::{Path, State};
use axum::Json;
use axum_extra::extract::CookieJar;
use http::{Request, StatusCode};
use serde_json::json;
use sqlx::PgPool;
use tracing::{error, info};
use crate::models::menu_models::categories::{Category, CategorySessionResponse, NewCategory};
use crate::services::menu_services::categories::{create_category_services, get_category_by_session_service, get_category_by_subdomain_services};
use crate::state::AppState;
use crate::utils::get_id::get_restaurant_id;

pub async fn create_category_handler(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(new_category): Json<NewCategory>,
) -> Result<Json<Category>, (StatusCode, Json<serde_json::Value>)> {
    let pool = state.pool;

    let restaurant_id = get_restaurant_id(&pool, &jar).await?;

    let category = create_category_services(&pool, restaurant_id, new_category)
        .await
        .map_err(|e| {
            error!("❌ Failed to create category: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                "error": "Failed to create category",
                "details": e.to_string()
            }))
            )
        })?;
    Ok(Json(category))
}

pub async fn get_category_by_subdomain_handler(
    State(state): State<AppState>,
    Path(restaurant_name): Path<String>,
) -> Result<Json<Vec<Category>>, (StatusCode, Json<serde_json::Value>)> {
    let pool = state.pool;

    let categories = get_category_by_subdomain_services(&pool, &restaurant_name)
        .await
        .map_err(|e| {
            error!("❌ Failed to get categories: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Failed to get categories",
                    "details": e.to_string()
                }))
            )
        })?;

    info!("✅ Found {} categories", categories.len());
    Ok(Json(categories))
}

pub async fn get_category_by_session_handler(
    State(state): State<AppState>,
    jar: CookieJar,
)->Result<Json<Vec<CategorySessionResponse>>, (StatusCode, Json<serde_json::Value>)>{
    let pool = state.pool;

    let restaurant_id = get_restaurant_id(&pool, &jar).await?;

    info!("🔍 Getting categories for restaurant: {}", restaurant_id);

    let category = get_category_by_session_service(&pool, restaurant_id)
        .await
        .map_err(|e| {
            error!("Failed to get category: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                "error": "Failed to get category",
                "details": e.to_string()
            }))
            )
        })?;
    Ok(Json(category))
}
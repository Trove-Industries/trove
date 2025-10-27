use axum::extract::{Path, State};
use axum::Json;
use http::StatusCode;
use serde_json::json;
use sqlx::PgPool;
use axum_extra::extract::CookieJar;
use tracing::error;
use crate::models::menu_models::categories::CategorySessionResponse;
use crate::models::menu_models::meal_groups::{MealGroup, NewMealGroup};
use crate::services::menu_services::categories::get_category_by_session_service;
use crate::services::menu_services::meal_groups::{create_meal_groups_service, get_meal_group_by_session_service, get_meal_group_by_subdomain_service};
use crate::state::AppState;
use crate::utils::get_id::get_restaurant_id;

pub async fn create_meal_group_handler(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(new_meal_group): Json<NewMealGroup>
)->Result<Json<MealGroup>, (StatusCode, Json<serde_json::Value>)>{
    let pool = state.pool;

    let restaurant_id = get_restaurant_id(&pool, &jar).await?;

    let meal_group  = create_meal_groups_service(&pool, new_meal_group.category_id, restaurant_id, new_meal_group)
        .await
        .map_err(|e| {
            tracing::error!("❌ Failed to create meal group: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Failed to create meal group",
                    "details": e.to_string()
                }))
            )
        })?;
    Ok(Json(meal_group))
}

pub async fn get_meal_group_by_subdomain_handler(
    State(state): State<AppState>,
    Path(restaurant_name): Path<String>
)->Result<Json<Vec<MealGroup>>, (StatusCode, Json<serde_json::Value>)>{
    let pool = state.pool;
    
    let meal_group = get_meal_group_by_subdomain_service(&pool, &restaurant_name)
        .await
        .map_err(|e| {
            tracing::error!("❌ Failed to get meal group: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Failed to get meal group",
                    "details": e.to_string()
                }))
            )
        })?;
    Ok(Json(meal_group))
}

pub async fn get_meal_group_by_session_handler(
    State(state): State<AppState>,
    jar: CookieJar,
)->Result<Json<Vec<MealGroup>>, (StatusCode, Json<serde_json::Value>)>{
    let pool = state.pool;

    let restaurant_id = get_restaurant_id(&pool, &jar).await?;

    let meal_group = get_meal_group_by_session_service(&pool, restaurant_id)
        .await
        .map_err(|e| {
            error!("Failed to get meal group: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                "error": "Failed to get meal group",
                "details": e.to_string()
            }))
            )
        })?;
    Ok(Json(meal_group))
}
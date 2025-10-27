use axum::extract::{Path, State};
use axum::Json;
use axum_extra::extract::CookieJar;
use http::StatusCode;
use serde_json::json;
use sqlx::{Error, PgPool};
use tracing::error;
use crate::models::menu_models::meals::{Meal, NewMeal};
use crate::services::menu_services::meals::{create_meals_service, get_meal_by_session_service, get_meal_by_subdomain_service};
use crate::state::AppState;
use crate::utils::get_id::get_restaurant_id;

pub async fn create_meal_handler(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(new_meal): Json<NewMeal>
)->Result<Json<Meal>,(StatusCode, Json<serde_json::Value>)>{
    let pool = state.pool;

    let restaurant_id = get_restaurant_id(&pool, &jar).await?;

    let meal = create_meals_service(&pool, new_meal.meal_group_id, restaurant_id, new_meal)
        .await
        .map_err(|e| {
            tracing::error!("❌ Failed to create meals: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Failed to create meal",
                    "details": e.to_string()
                }))
            )
        })?;
    Ok(Json(meal))
}

pub async fn get_meal_by_subdomain_handler(
    State(state): State<AppState>,
    Path(restaurant_name): Path<String>
)->Result<Json<Vec<Meal>>, (StatusCode, Json<serde_json::Value>)>{
    let pool = state.pool;
    
    let meal = get_meal_by_subdomain_service(&pool, &restaurant_name)
        .await
        .map_err(|e| {
            tracing::error!("❌ Failed to get meal: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Failed to get meal",
                    "details": e.to_string()
                }))
            )
        })?;
    Ok(Json(meal))
}

pub async fn get_meal_by_session_handler(
    State(state): State<AppState>,
    jar: CookieJar,
) -> Result<Json<Vec<Meal>>, (StatusCode, Json<serde_json::Value>)> {
    let pool = state.pool;
    
    let restaurant_id = get_restaurant_id(&pool, &jar).await?;

    let meal = get_meal_by_session_service(&pool, restaurant_id)
        .await
        .map_err(|e| {
            error!("Failed to get meal: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Failed to get meal",
                    "details": e.to_string()
                }))
            )
        })?;

    Ok(Json(meal))
}
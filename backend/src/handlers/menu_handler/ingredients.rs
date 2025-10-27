use axum::extract::{Path, State};
use axum::Json;
use axum_extra::extract::CookieJar;
use http::StatusCode;
use serde_json::json;
use sqlx::PgPool;
use tracing::error;
use crate::models::menu_models::categories::Category;
use crate::models::menu_models::ingredient::{Ingredient, NewIngredient};
use crate::services::menu_services::ingredients::{create_ingredients_service, get_ingredient_by_session_service, get_ingredient_by_subdomain_service};
use crate::state::AppState;
use crate::utils::get_id::get_restaurant_id;

pub async fn create_ingredient_handler(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(new_ingredient): Json<NewIngredient>
)-> Result<Json<Ingredient>, (StatusCode, Json<serde_json::Value>)>{
    let pool = state.pool;
    
    let restaurant_id = get_restaurant_id(&pool, &jar).await?;

    let ingredient = create_ingredients_service(&pool, new_ingredient.meal_id, restaurant_id, new_ingredient)
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
    Ok(Json(ingredient))
}

pub async fn get_ingredient_by_subdomain_handler(
    State(state): State<AppState>,
    Path(restaurant_name): Path<String>
)->Result<Json<Vec<Ingredient>>, (StatusCode, Json<serde_json::Value>)>{
    let pool = state.pool;
    
    let ingredient = get_ingredient_by_subdomain_service(&pool, &restaurant_name)
        .await
        .map_err(|e| {
            tracing::error!("❌ Failed to get ingredients {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Failed to get ingredients",
                    "details": e.to_string()
                }))
            )
        })?;
    Ok(Json(ingredient))
}

pub async fn get_ingredient_by_session_handler(
    State(state): State<AppState>,
    jar: CookieJar,
)->Result<Json<Vec<Ingredient>>, (StatusCode, Json<serde_json::Value>)>{
    let pool = state.pool;

    let restaurant_id = get_restaurant_id(&pool, &jar).await?;

    let ingredient = get_ingredient_by_session_service(&pool, restaurant_id)
        .await
        .map_err(|e| {
            error!("Failed to get ingredient: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                "error": "Failed to get ingredient",
                "details": e.to_string()
            }))
            )
        })?;
    Ok(Json(ingredient))
}
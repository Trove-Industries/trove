use axum::extract::{Path, State};
use axum::Json;
use sqlx::PgPool;
use crate::models::menu_models::ingredient::{Ingredient, NewIngredient};
use crate::services::menu_services::ingredients::{create_ingredients_service, get_ingredient_service};

pub async fn create_ingredient_handler(
    State(pool): State<PgPool>,
    Json(new_ingredient): Json<NewIngredient>
)->Result<Json<Ingredient>, (axum::http::StatusCode, String)>{
    let category = create_ingredients_service(&pool, new_ingredient)
        .await
        .map_err(|e|(axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(category))
}

pub async fn get_ingredient_handler(
    State(pool): State<PgPool>,
    Path(restaurant_name): Path<String>
)->Result<Json<Vec<Ingredient>>, (axum::http::StatusCode, String)>{
    let ingredient = get_ingredient_service(&pool, &restaurant_name)
        .await
        .map_err(|e|(axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(ingredient))
}
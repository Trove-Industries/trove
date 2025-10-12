use axum::extract::{Path, State};
use axum::Json;
use sqlx::PgPool;
use crate::models::menu_models::meals::{Meal, NewMeal};
use crate::services::menu_services::meals::{create_meals_service, get_meal_service};

pub async fn create_meal_handler(
    State(pool): State<PgPool>,
    Json(new_meal): Json<NewMeal>
)->Result<Json<Meal>,(axum::http::StatusCode, String)>{
    let meal = create_meals_service(&pool, new_meal)
        .await
        .map_err(|e|(axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(meal))
}

pub async fn get_meal_handler(
    State(pool): State<PgPool>,
    Path(restaurant_name): Path<String>
)->Result<Json<Vec<Meal>>, (axum::http::StatusCode, String)>{
    let meal = get_meal_service(&pool, &restaurant_name)
        .await
        .map_err(|e|(axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(meal))
}
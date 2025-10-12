use axum::extract::{Path, State};
use axum::Json;
use sqlx::PgPool;
use crate::models::menu_models::meal_groups::{MealGroup, NewMealGroup};
use crate::services::menu_services::meal_groups::{create_meal_groups_service, get_meal_groups_service};

pub async fn create_meal_group_handler(
    State(pool): State<PgPool>,
    Json(new_meal_group): Json<NewMealGroup>
)->Result<Json<MealGroup>, (axum::http::StatusCode, String)>{
    let meal_group = create_meal_groups_service(&pool, new_meal_group)
        .await
        .map_err(|e|(axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(meal_group))
}

pub async fn get_meal_group_handler(
    State(pool): State<PgPool>,
    Path(restaurant_name): Path<String>
)->Result<Json<Vec<MealGroup>>, (axum::http::StatusCode, String)>{
    let meal_group = get_meal_groups_service(&pool, &restaurant_name)
        .await
        .map_err(|e|(axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(meal_group))
}
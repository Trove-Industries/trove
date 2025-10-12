use axum::extract::{Path, State};
use axum::Json;
use sqlx::PgPool;
use crate::models::restaurants_models::restaurants::{NewRestaurant, Restaurant};
use crate::services::restaurant_services::restaurant::{create_restaurant_service, get_restaurant_service};

pub async fn create_restaurant_handler(
    State(pool): State<PgPool>,
    Json(new_restaurant): Json<NewRestaurant>
)->Result<Json<Restaurant>, (axum::http::StatusCode, String)>{
    let restaurant = create_restaurant_service(&pool, new_restaurant)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(restaurant))
}

pub async fn get_restaurant_handler(
    State(pool): State<PgPool>,
    Path(restaurant_name): Path<String>
)->Result<Json<Vec<Restaurant>>, (axum::http::StatusCode, String)>{
    let restaurant = get_restaurant_service(&pool, &restaurant_name)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(restaurant))
}
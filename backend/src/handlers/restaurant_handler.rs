use axum::{extract::{Path,State}, Json};
use sqlx::PgPool;
use crate::models::restaurant_models::{NewRestaurant, Restaurant, RestaurantName};
use crate::services::restaurant_service;


pub async fn register_restaurant(
    State(pool): State<PgPool>,
    Json(new_restaurant): Json<NewRestaurant>
) ->Result<Json<Restaurant>, (axum::http::StatusCode, String)>{
    let restaurant = restaurant_service::register_restaurant(&pool, new_restaurant)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(restaurant))
}

pub async fn validate_restaurant(
    State(pool): State<PgPool>,
    Path(restaurant_name): Path<String>
) ->Result<Json<Vec<RestaurantName>>, (axum::http::StatusCode,String)>{
    let name = restaurant_service::validate_restaurant(&pool, restaurant_name)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(name))
}
use axum::{extract::{Path,State}, Json};
use sqlx::PgPool;
use crate::models::menu_item::{MenuItem, NewMenuItem, RestaurantName};
use crate::services::menu_service;

pub async fn create_menu(
    State(pool): State<PgPool>,
    Json(new_item): Json<NewMenuItem>
) ->Result<Json<MenuItem>,(axum::http::StatusCode, String)>{
    let item = menu_service::create_menu(&pool,new_item)
        .await
        .map_err(|e|(axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(item))
}

pub async fn get_menu(
    State(pool): State<PgPool>,
    Path(restaurant_name): Path<String>,
) ->Result<Json<Vec<MenuItem>>, (axum::http::StatusCode, String)>{
    let menu = menu_service::get_menu(&pool, restaurant_name)
        .await
        .map_err(|e|(axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(menu))
}

pub async fn validate_restaurant(
    State(pool): State<PgPool>,
    Path(restaurant_name): Path<String>
) ->Result<Json<Vec<RestaurantName>>, (axum::http::StatusCode,String)>{
    let name = menu_service::validate_restaurant(&pool, restaurant_name)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(name))
}


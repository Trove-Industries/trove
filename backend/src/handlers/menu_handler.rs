use axum::{extract::{Path,State}, Json};
use sqlx::PgPool;
use crate::models::menu_item::{MenuItem, NewMenuItem};
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
    Path(restaurant_id): Path<i32>,
) ->Result<Json<Vec<MenuItem>>, (axum::http::StatusCode, String)>{
    let menu = menu_service::get_menu(&pool, restaurant_id)
        .await
        .map_err(|e|(axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(menu))
}


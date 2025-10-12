use axum::extract::{Path, State};
use axum::Json;
use sqlx::PgPool;
use crate::models::menu_models::sizes::{NewSize, Size};
use crate::services::menu_services::sizes::{create_sizes_service, get_size_service};

pub async fn create_size_handler(
    State(pool): State<PgPool>,
    Json(new_size): Json<NewSize>
)->Result<Json<Size>, (axum::http::StatusCode, String)>{
    let size = create_sizes_service(&pool, new_size)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(size))
}

pub async fn get_size_handler(
    State(pool): State<PgPool>,
    Path(restaurant_name): Path<String>
)->Result<Json<Vec<Size>>, (axum::http::StatusCode, String)>{
    let size = get_size_service(&pool, &restaurant_name)
        .await
        .map_err(|e|(axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(size))
}
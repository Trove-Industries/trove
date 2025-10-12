use axum::extract::{Path, State};
use axum::Json;
use sqlx::PgPool;
use crate::models::menu_models::categories::{Category, NewCategory};
use crate::services::menu_services::categories::{create_category_services, get_category_services};

pub async fn create_category_handler(
    State(pool): State<PgPool>,
    Json(new_category): Json<NewCategory>
)->Result<Json<Category>, (axum::http::StatusCode, String)>{
    let category = create_category_services(&pool, new_category)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(category))
}

pub async fn get_category_handler(
    State(pool): State<PgPool>,
    Path(restaurant_name): Path<String>,
)->Result<Json<Vec<Category>>, (axum::http::StatusCode, String)>{
    let category = get_category_services(&pool, &restaurant_name)
        .await
        .map_err(|e|(axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(category))
}


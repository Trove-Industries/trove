use axum::extract::{Path, State};
use axum::Json;
use sqlx::PgPool;
use crate::models::menu_models::pairings::{NewPairing, Pairing};
use crate::services::menu_services::pairings::{create_pairings_service, get_pairings_service};

pub async fn create_pairing_handler(
    State(pool): State<PgPool>,
    Json(new_pairing): Json<NewPairing>
)->Result<Json<Pairing>, (axum::http::StatusCode, String)>{
    let pairing = create_pairings_service(&pool, new_pairing)
        .await
        .map_err(|e|(axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(pairing))
}

pub async fn get_pairing_handler(
    State(pool): State<PgPool>,
    Path(restaurant_name): Path<String>
)->Result<Json<Vec<Pairing>>, (axum::http::StatusCode ,String)>{
    let pairing = get_pairings_service(&pool, &restaurant_name)
        .await
        .map_err(|e|(axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(pairing))
}
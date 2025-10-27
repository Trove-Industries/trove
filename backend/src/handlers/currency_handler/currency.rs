use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use http::StatusCode;
use sqlx::PgPool;
use crate::models::currency_models::currency::NewCurrency;
use crate::services::currency_services::currency::{create_currency_service, get_all_currencies_service, get_currency_by_iso_service};
use crate::state::AppState;

/// POST /currency/create
pub async fn create_currency_handler(
    State(state): State<AppState>,
    Json(new_currency): Json<NewCurrency>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let pool = state.pool;
    
    let currency = create_currency_service(&pool, new_currency)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;
    Ok((StatusCode::CREATED, Json(currency)))
}

/// GET /currency/{iso}
pub async fn get_currency_by_iso_handler(
    State(state): State<AppState>,
    Path(iso): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let pool = state.pool;
    
    let currency = get_currency_by_iso_service(&pool, &iso)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    match currency {
        Some(data) => Ok((StatusCode::OK, Json(data))),
        None => Err((StatusCode::NOT_FOUND, format!("Currency with ISO '{}' not found", iso))),
    }
}

/// GET /currency/all
pub async fn get_all_currencies_handler(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let pool = state.pool;
    
    let currencies = get_all_currencies_service(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok((StatusCode::OK, Json(currencies)))
}

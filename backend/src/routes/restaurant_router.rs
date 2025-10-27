use axum::{Router, routing::{get, post}};
use sqlx::PgPool;
use crate::handlers::restaurant_handler::restaurant::{create_restaurant_handler, get_restaurant_by_session_handler, get_restaurant_handler};
use crate::state::AppState;

pub fn restaurant_routes() -> Router<AppState> {
    Router::new()
        .route("/create-restaurant", post(create_restaurant_handler))
        .route("/get-restaurant/{restaurant_name}", get(get_restaurant_handler))
        .route("/restore-restaurant-session", get(get_restaurant_by_session_handler))
}
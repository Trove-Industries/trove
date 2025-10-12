use axum::{Router, routing::{get, post}};
use sqlx::PgPool;
use crate::handlers::restaurant_handler::restaurant::{create_restaurant_handler, get_restaurant_handler};

// menu_router
pub fn restaurant_routes() -> Router<PgPool> {
    Router::new()
        .route("/create-restaurant", post(create_restaurant_handler))
        .route("/get-restaurant/{restaurant_name}", get(get_restaurant_handler))
}
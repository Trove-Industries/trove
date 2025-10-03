use axum::{Router, routing::{get, post}};
use sqlx::PgPool;
use crate::handlers::restaurant_handler::{validate_restaurant};

// menu_router
pub fn restaurant_routes() -> Router<PgPool> {
    Router::new()
        .route("/menu-items/validate/{restaurant_name}", get(validate_restaurant))
}
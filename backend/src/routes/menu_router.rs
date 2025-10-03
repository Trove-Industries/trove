use axum::{Router, routing::{get, post}};
use sqlx::PgPool;
use crate::handlers::menu_handler::{get_menu, create_menu,validate_restaurant};

// menu_router
pub fn menu_routes() -> Router<PgPool> {
    Router::new()
        .route("/menu-items", post(create_menu))
        .route("/menu-items/{restaurant_name}", get(get_menu))
        .route("/menu-items/validate/{restaurant_name}", get(validate_restaurant))
}
use axum::{Router, routing::{get, post}};
use sqlx::PgPool;
use crate::handlers::menu_handler::categories::{create_category_handler, get_category_handler};
use crate::handlers::menu_handler::ingredients::{create_ingredient_handler, get_ingredient_handler};
use crate::handlers::menu_handler::meal_groups::{create_meal_group_handler, get_meal_group_handler};
use crate::handlers::menu_handler::meals::{create_meal_handler, get_meal_handler};
use crate::handlers::menu_handler::pairings::{create_pairing_handler, get_pairing_handler};
use crate::handlers::menu_handler::sizes::{create_size_handler, get_size_handler};

// menu_router
pub fn menu_routes() -> Router<PgPool> {
    Router::new()
        .route("/create-category", post(create_category_handler))
        .route("/create-ingredient", post(create_ingredient_handler))
        .route("/create-meal-group", post(create_meal_group_handler))
        .route("/create-meal", post(create_meal_handler))
        .route("/create-pairing", post(create_pairing_handler))
        .route("/create-meal-size", post(create_size_handler))

        .route("/get-category/{restaurant_name}", get(get_category_handler))
        .route("/get-meal-group/{restaurant_name}", get(get_meal_group_handler))
        .route("/get-meal/{restaurant_name}", get(get_meal_handler))
        .route("/get-size/{restaurant_name}", get(get_size_handler))
        .route("/get-pairing/{restaurant_name}", get(get_pairing_handler))
        .route("/get-ingredient/{restaurant_name}", get(get_ingredient_handler))
}

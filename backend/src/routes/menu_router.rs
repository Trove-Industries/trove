use axum::{Router, routing::{get, post}};
use sqlx::PgPool;
use crate::handlers::menu_handler::categories::{create_category_handler, get_category_by_session_handler, get_category_by_subdomain_handler};
use crate::handlers::menu_handler::ingredients::{create_ingredient_handler, get_ingredient_by_session_handler, get_ingredient_by_subdomain_handler};
use crate::handlers::menu_handler::meal_groups::{create_meal_group_handler, get_meal_group_by_session_handler, get_meal_group_by_subdomain_handler};
use crate::handlers::menu_handler::meals::{create_meal_handler, get_meal_by_session_handler, get_meal_by_subdomain_handler};
use crate::handlers::menu_handler::pairings::{create_pairings_handler, get_pairings_by_session_handler, get_pairings_by_subdomain_handler};
use crate::handlers::menu_handler::sizes::{create_size_handler, get_size_by_session_handler, get_size_by_subdomain_handler};
use crate::state::AppState;

// menu_router
pub fn menu_routes() -> Router<AppState> {
    Router::new()
        // app
        .route("/create-category", post(create_category_handler))
        .route("/create-ingredient", post(create_ingredient_handler))
        .route("/create-meal-group", post(create_meal_group_handler))
        .route("/create-meal", post(create_meal_handler))
        .route("/create-pairing", post(create_pairings_handler))
        .route("/create-meal-size", post(create_size_handler))

        // app
        .route("/restore-category-session", get(get_category_by_session_handler))
        .route("/restore-meal-group-session", get(get_meal_group_by_session_handler))
        .route("/restore-meal-session", get(get_meal_by_session_handler))
        .route("/restore-meal-size-session", get(get_size_by_session_handler))
        .route("/restore-pairing-session", get(get_pairings_by_session_handler))
        .route("/restore-ingredient-session", get(get_ingredient_by_session_handler))


        // browser url
        .route("/get-category/{restaurant_name}", get(get_category_by_subdomain_handler))
        .route("/get-meal-group/{restaurant_name}", get(get_meal_group_by_subdomain_handler))
        .route("/get-meal/{restaurant_name}", get(get_meal_by_subdomain_handler))
        .route("/get-size/{restaurant_name}", get(get_size_by_subdomain_handler))
        .route("/get-pairing/{restaurant_name}", get(get_pairings_by_subdomain_handler))
        .route("/get-ingredient/{restaurant_name}", get(get_ingredient_by_subdomain_handler))
}

use sqlx::PgPool;
use crate::db::menu_queries::categories::get_categories_by_subdomain_query;
use crate::db::menu_queries::ingredients::get_ingredient_by_subdomain_query;
use crate::db::menu_queries::meal_groups::get_meal_group_by_subdomain_query;
use crate::db::menu_queries::meals::get_meal_by_subdomain_query;
use crate::db::menu_queries::pairings::get_pairing_by_subdomain_query;
use crate::db::menu_queries::sizes::get_size_by_subdomain_query;
use crate::db::restaurants_queries::restaurants::get_restaurant_query;
use crate::models::template_models::menu_template_model::FullMenuData;


pub async fn get_full_menu_data(pool: &PgPool, restaurant_name: &String) -> Result<FullMenuData, (axum::http::StatusCode, String)>{
    let restaurant = get_restaurant_query(pool, restaurant_name)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, format!("restaurant query error: {}", e)))?;

    let categories = get_categories_by_subdomain_query(pool, restaurant_name)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, format!("categories query error: {}", e)))?;

    let meal_groups = get_meal_group_by_subdomain_query(pool, restaurant_name)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, format!("meal groups query error: {}", e)))?;

    let meals = get_meal_by_subdomain_query(pool, restaurant_name)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, format!("meals query error: {}", e)))?;

    let sizes = get_size_by_subdomain_query(pool, restaurant_name)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, format!("sizes query error: {}", e)))?;

    let pairings = get_pairing_by_subdomain_query(pool, restaurant_name)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, format!("pairings query error: {}", e)))?;

    let ingredients = get_ingredient_by_subdomain_query(pool, restaurant_name)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, format!("ingredients query error: {}", e)))?;

    Ok(FullMenuData {
        restaurant,
        categories,
        meal_groups,
        meals,
        sizes,
        pairings,
        ingredients,
    })
}

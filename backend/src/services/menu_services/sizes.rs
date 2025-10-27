use sqlx::PgPool;
use crate::db::menu_queries::sizes::{create_size_query, get_size_by_session_query, get_size_by_subdomain_query};
use crate::models::menu_models::sizes::{NewSize, Size};

pub async fn create_sizes_service(
    pool: &PgPool,
    meal_id: i32,
    restaurant_id: i32,
    new_size: NewSize,
)->Result<Size, sqlx::Error>{
    create_size_query(pool, meal_id, restaurant_id, new_size).await
}

pub async fn get_size_by_subdomain_service(
    pool: &PgPool,
    restaurant_name: &String,
)->Result<Vec<Size>, sqlx::Error>{
    get_size_by_subdomain_query(pool, restaurant_name).await
}

pub async fn get_size_by_session_service(
    pool: &PgPool,
    restaurant_id: i32,
)->Result<Vec<Size>, sqlx::Error>{
    get_size_by_session_query(pool, restaurant_id).await
}
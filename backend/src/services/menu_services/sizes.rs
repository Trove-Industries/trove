use sqlx::PgPool;
use crate::db::menu_queries::sizes::{create_size_query, get_size_query};
use crate::models::menu_models::sizes::{NewSize, Size};

pub async fn create_sizes_service(
    pool: &PgPool,
    new_size: NewSize,
)->Result<Size, sqlx::Error>{
    create_size_query(pool, new_size).await
}

pub async fn get_size_service(
    pool: &PgPool,
    restaurant_name: &String,
)->Result<Vec<Size>, sqlx::Error>{
    get_size_query(pool,restaurant_name).await
}
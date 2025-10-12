use sqlx::{Error, PgPool};
use crate::db::menu_queries::categories::{create_category_query, get_categories_query};
use crate::models::menu_models::categories::{Category, NewCategory};

pub async  fn create_category_services(
    pool: &PgPool,
    new_category: NewCategory
)->Result<Category, sqlx::Error>{
    create_category_query(pool, new_category).await
}

pub async fn get_category_services(
    pool: &PgPool,
    restaurant_name: &String
)->Result<Vec<Category>, sqlx::Error>{
    get_categories_query(pool, restaurant_name).await
}
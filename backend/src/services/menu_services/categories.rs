use sqlx::{PgPool};
use uuid::Uuid;
use crate::db::menu_queries::categories::{create_category_query, get_categories_by_subdomain_query, get_category_by_session_query};
use crate::models::menu_models::categories::{Category, CategorySessionResponse, NewCategory};

pub async  fn create_category_services(
    pool: &PgPool,
    restaurant_id: i32,
    new_category: NewCategory
) ->Result<Category, sqlx::Error>{
    create_category_query(pool, restaurant_id, new_category).await
}

pub async fn get_category_by_subdomain_services(
    pool: &PgPool,
    restaurant_name: &String
)->Result<Vec<Category>, sqlx::Error>{
    get_categories_by_subdomain_query(pool, restaurant_name).await
}

pub async fn get_category_by_session_service(
    pool: &PgPool,
    restaurant_id: i32,
) ->Result<Vec<CategorySessionResponse>, sqlx::Error>{
    get_category_by_session_query(pool, restaurant_id).await
}
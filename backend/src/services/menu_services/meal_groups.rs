use sqlx::PgPool;
use crate::db::menu_queries::meal_groups::{create_meal_group_query, get_meal_group_by_session_query, get_meal_group_by_subdomain_query};
use crate::models::menu_models::categories::CategorySessionResponse;
use crate::models::menu_models::meal_groups::{MealGroup, NewMealGroup};

pub async fn create_meal_groups_service(
    pool: &PgPool,
    category_id: i32,
    restaurant_id: i32,
    new_meal_group: NewMealGroup
)->Result<MealGroup, sqlx::Error>{
    create_meal_group_query(pool, category_id, restaurant_id, new_meal_group).await
}

pub async fn get_meal_group_by_subdomain_service(
    pool: &PgPool,
    restaurant_name: &String,
)->Result<Vec<MealGroup>, sqlx::Error>{
    get_meal_group_by_subdomain_query(pool, restaurant_name).await
}

pub async fn get_meal_group_by_session_service(
    pool: &PgPool,
    restaurant_id: i32,
)->Result<Vec<MealGroup>, sqlx::Error>{
    get_meal_group_by_session_query(pool, restaurant_id).await
}
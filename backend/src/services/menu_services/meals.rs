use sqlx::PgPool;
use crate::db::menu_queries::meals::{create_meal_query, get_meal_by_session_query, get_meal_by_subdomain_query};
use crate::models::menu_models::meals::{Meal, NewMeal};

pub async fn create_meals_service(
    pool: &PgPool,
    meal_group_id: i32,
    restaurant_id: i32,
    new_meal: NewMeal
)->Result<Meal, sqlx::Error>{
    create_meal_query(pool, meal_group_id, restaurant_id, new_meal).await
}

pub async fn get_meal_by_subdomain_service(
    pool: &PgPool,
    restaurant_name: &String
)->Result<Vec<Meal>, sqlx::Error>{
    get_meal_by_subdomain_query(pool, restaurant_name).await
}

pub async fn get_meal_by_session_service(
    pool: &PgPool,
    restaurant_id: i32,
)->Result<Vec<Meal>, sqlx::Error>{
    get_meal_by_session_query(pool, restaurant_id).await
}
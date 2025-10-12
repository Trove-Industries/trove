use sqlx::PgPool;
use crate::db::menu_queries::meals::{create_meal_query, get_meal_query};
use crate::models::menu_models::meals::{Meal, NewMeal};

pub async fn create_meals_service(
    pool: &PgPool,
    new_meal: NewMeal
)->Result<Meal, sqlx::Error>{
    create_meal_query(pool, new_meal).await
}

pub async fn get_meal_service(
    pool: &PgPool,
    restaurant_name: &String
)->Result<Vec<Meal>, sqlx::Error>{
    get_meal_query(pool, restaurant_name).await
}
use sqlx::PgPool;
use crate::db::menu_queries::ingredients::{create_ingredient_query, get_ingredient_by_session_query, get_ingredient_by_subdomain_query};
use crate::models::menu_models::ingredient::{Ingredient, NewIngredient};

pub async fn create_ingredients_service(
    pool: &PgPool,
    meal_id: i32,
    restaurant_id: i32,
    new_ingredient: NewIngredient
)->Result<Ingredient, sqlx::Error>{
    create_ingredient_query(pool, meal_id, restaurant_id, new_ingredient).await
}

pub async fn get_ingredient_by_subdomain_service(
    pool: &PgPool,
    restaurant_name: &String
)->Result<Vec<Ingredient>, sqlx::Error>{
    get_ingredient_by_subdomain_query(pool, restaurant_name).await
}

pub async fn get_ingredient_by_session_service(
    pool: &PgPool,
    restaurant_id: i32,
)->Result<Vec<Ingredient>, sqlx::Error>{
    get_ingredient_by_session_query(pool, restaurant_id).await
}
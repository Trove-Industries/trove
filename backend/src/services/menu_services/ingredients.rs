use sqlx::PgPool;
use crate::db::menu_queries::ingredients::{create_ingredient_query, get_ingredient_query};
use crate::models::menu_models::ingredient::{Ingredient, NewIngredient};

pub async fn create_ingredients_service(
    pool: &PgPool,
    new_ingredient: NewIngredient
)->Result<Ingredient, sqlx::Error>{
    create_ingredient_query(pool, new_ingredient).await
}

pub async fn get_ingredient_service(
    pool: &PgPool,
    restaurant_name: &String
)->Result<Vec<Ingredient>, sqlx::Error>{
    get_ingredient_query(pool, restaurant_name).await
}
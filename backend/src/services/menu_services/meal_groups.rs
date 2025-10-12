use sqlx::PgPool;
use crate::db::menu_queries::meal_groups::{create_meal_group_query, get_meal_group_query};
use crate::models::menu_models::meal_groups::{MealGroup, NewMealGroup};

pub async fn create_meal_groups_service(
    pool: &PgPool,
    new_meal_group: NewMealGroup
)->Result<MealGroup, sqlx::Error>{
    create_meal_group_query(pool, new_meal_group).await
}

pub async fn get_meal_groups_service(
    pool: &PgPool,
    restaurant_name: &String,
)->Result<Vec<MealGroup>, sqlx::Error>{
    get_meal_group_query(pool, restaurant_name).await
}
use sqlx::{Error, PgPool};
use crate::models::menu_models::ingredient::{Ingredient, NewIngredient};

pub async fn create_ingredient_query(
    pool: &PgPool,
    meal_id: i32,
    restaurant_id: i32,
    new_ingredient: NewIngredient
)->Result<Ingredient, Error>{
    let new_ingredient = sqlx::query_as::<_,Ingredient>(
        r#"
                INSERT INTO ingredients(meal_id, restaurant_id, ingredient_name, ingredient_image)
                VALUES ($1, $2, $3, $4)
                RETURNING id, meal_id, restaurant_id, ingredient_name, ingredient_image
            "#
    )
        .bind(meal_id)
        .bind(restaurant_id)
        .bind(new_ingredient.ingredient_name)
        .bind(new_ingredient.ingredient_image)
        .persistent(false)
        .fetch_one(pool)
        .await?;
    Ok(new_ingredient)
}

pub async fn get_ingredient_by_subdomain_query(
    pool: &PgPool,
    restaurant_name: &String
)->Result<Vec<Ingredient>, Error>{
    let ingredient = sqlx::query_as::<_,Ingredient>(
        r#"
                SELECT i.id, i.meal_id, i.restaurant_id, i.ingredient_name, i.ingredient_image
                FROM ingredients i
                JOIN restaurants r ON i.restaurant_id = r.id
                WHERE r.restaurant_name ILIKE $1
            "#
    )
        .bind(restaurant_name)
        .persistent(false)
        .fetch_all(pool)
        .await?;
    Ok(ingredient)
}

pub async fn get_ingredient_by_session_query(
    pool: &PgPool,
    restaurant_id: i32,
)->Result<Vec<Ingredient>, Error>{
    let ingredients = sqlx::query_as::<_,Ingredient>(
        r#"
                SELECT id, meal_id, restaurant_id, ingredient_name, ingredient_image
                FROM ingredients
                WHERE restaurant_id = $1
            "#
    )
        .bind(restaurant_id)
        .persistent(false)
        .fetch_all(pool)
        .await?;
    Ok(ingredients)
}
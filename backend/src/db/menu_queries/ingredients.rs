use sqlx::{Error, PgPool};
use crate::models::menu_models::ingredient::{Ingredient, NewIngredient};

pub async fn create_ingredient_query(
    pool: &PgPool,
    new_ingredient: NewIngredient
)->Result<Ingredient, Error>{
    let new_ingredient = sqlx::query_as::<_,Ingredient>(
        r#"
                INSERT INTO ingredients(meal_id, restaurant_id, ingredient_name, ingredient_image)
                VALUES (
                        (
                            SELECT m.id
                            FROM meals m
                            JOIN restaurants r ON m.restaurant_id = r.id
                            WHERE m.meal_name ILIKE $2 AND r.restaurant_name = $1
                        ),
                        (
                            SELECT id
                            FROM restaurants
                            WHERE restaurant_name = $1
                        ),
                        $3, $4
                )
                RETURNING id, meal_id, restaurant_id, ingredient_name, ingredient_image
            "#
    )
        .bind(new_ingredient.restaurant_name)
        .bind(new_ingredient.meal_name)
        .bind(new_ingredient.ingredient_name)
        .bind(new_ingredient.ingredient_image)
        .persistent(false)
        .fetch_one(pool)
        .await?;
    Ok(new_ingredient)
}

pub async fn get_ingredient_query(
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
        .fetch_all(pool)
        .await?;
    Ok(ingredient)
}
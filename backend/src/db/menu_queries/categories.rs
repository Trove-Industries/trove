use sqlx::{Error, PgPool};
use crate::models::menu_models::categories::{Category, CategorySessionResponse, NewCategory};

pub async fn create_category_query(
    pool: &PgPool,
    restaurant_id: i32,
    new_category: NewCategory,
) ->Result<Category, Error> {
    let new_category = sqlx::query_as::<_,Category>(
        r#"
                INSERT INTO categories (restaurant_id, category_name, category_icon)
                VALUES ($1, $2, $3)
                RETURNING id, restaurant_id, category_name, category_icon
            "#
    )
        .bind(restaurant_id)
        .bind(new_category.category_name)
        .bind(new_category.category_icon)
        .persistent(false)
        .fetch_one(pool)
        .await?;
    Ok(new_category)
}

pub async fn get_categories_by_subdomain_query(
    pool: &PgPool,
    restaurant_name: &String,
)->Result<Vec<Category>, Error>{
    let categories = sqlx::query_as::<_,Category>(
        r#"
                SELECT c.id, c.restaurant_id, c.category_name, c.category_icon
                FROM categories c
                INNER JOIN restaurants r ON c.restaurant_id = r.id
                WHERE r.restaurant_name ILIKE $1
            "#
    )
        .bind(restaurant_name)
        .persistent(false)
        .fetch_all(pool)
        .await?;
    Ok(categories)
}

pub async fn get_category_by_session_query(
    pool: &PgPool,
    restaurant_id: i32,
) ->Result<Vec<CategorySessionResponse>, Error>{
    let categories = sqlx::query_as::<_,CategorySessionResponse>(
        r#"
                SELECT id, category_name, category_icon
                FROM categories
                WHERE restaurant_id = $1
            "#
    )
        .bind(restaurant_id)
        .persistent(false)
        .fetch_all(pool)
        .await?;
    Ok(categories)
}
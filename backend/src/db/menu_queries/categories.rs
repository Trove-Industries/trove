use sqlx::{Error, Execute, PgPool};
use crate::models::menu_models::categories::{Category, NewCategory};

pub async fn create_category_query(
    pool: &PgPool,
    new_category: NewCategory,
) ->Result<Category, Error> {
    let new_category = sqlx::query_as::<_,Category>(
        r#"
                INSERT INTO categories (restaurant_id, category_name, category_icon)
                VALUES (
                        (SELECT id FROM restaurants WHERE restaurant_name = $1),
                        $2, $3
                )
                RETURNING id, restaurant_id, category_name, category_icon
            "#
    )
        .bind(new_category.restaurant_name)
        .bind(new_category.category_name)
        .bind(new_category.category_icon)
        .persistent(false)
        .fetch_one(pool)
        .await?;
    Ok(new_category)
}

pub async fn get_categories_query(
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



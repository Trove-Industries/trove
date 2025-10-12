use sqlx::{Error, PgPool};
use crate::models::menu_models::pairings::{NewPairing, Pairing};

pub async fn create_pairing_query(
    pool: &PgPool,
    new_pairing: NewPairing
)->Result<Pairing, Error>{
    let new_pairing = sqlx::query_as::<_,Pairing>(
        r#"
                INSERT INTO pairings(meal_id, restaurant_id, pairing_name, pairing_image, pairing_price)
                VALUES (
                    (
                        SELECT m.id
                        FROM meals m
                        JOIN restaurants r ON m.restaurant_id = r.id
                        WHERE m.meal_name ILIKE $2 AND r.restaurant_name ILIKE $1
                    ),
                    (
                        SELECT r.id FROM restaurants r WHERE r.restaurant_name ILIKE $1
                    ),
                    $3, $4, $5
                )
                RETURNING id, meal_id, restaurant_id, pairing_name, pairing_image, pairing_price
            "#
    )
        .bind(new_pairing.restaurant_name)
        .bind(new_pairing.meal_name)
        .bind(new_pairing.pairing_name)
        .bind(new_pairing.pairing_image)
        .bind(new_pairing.pairing_price)
        .persistent(false)
        .fetch_one(pool)
        .await?;
    Ok(new_pairing)
}

pub async fn get_pairing_query(
    pool: &PgPool,
    restaurant_name: &String,
)->Result<Vec<Pairing>, Error>{
    let pairing = sqlx::query_as::<_,Pairing>(
        r#"
               SELECT p.id, p.meal_id, p.restaurant_id, p.pairing_name, p.pairing_image, p.pairing_price
               FROM pairings p
               JOIN restaurants r ON p.restaurant_id = r.id
               WHERE r.restaurant_name ILIKE $1
            "#
    )
        .bind(restaurant_name)
        .persistent(false)
        .fetch_all(pool)
        .await?;
    Ok(pairing)
}


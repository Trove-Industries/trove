use sqlx::PgPool;
use crate::db::menu_queries::pairings::{create_pairing_query, get_pairing_query};
use crate::models::menu_models::pairings::{NewPairing, Pairing};

pub async fn create_pairings_service(
    pool: &PgPool,
    new_pairing: NewPairing,
)->Result<Pairing, sqlx::Error>{
    create_pairing_query(pool, new_pairing).await
}

pub async fn get_pairings_service(
    pool: &PgPool,
    restaurant_name: &String
)->Result<Vec<Pairing>, sqlx::Error>{
    get_pairing_query(pool, restaurant_name).await
}
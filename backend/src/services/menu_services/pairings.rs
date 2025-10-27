use sqlx::PgPool;
use crate::db::menu_queries::pairings::{create_pairing_query, get_pairing_by_session_query, get_pairing_by_subdomain_query};
use crate::models::menu_models::pairings::{NewPairing, Pairing};

pub async fn create_pairings_service(
    pool: &PgPool,
    meal_id: i32,
    restaurant_id: i32,
    new_pairing: NewPairing,
)->Result<Pairing, sqlx::Error>{
    create_pairing_query(pool, meal_id, restaurant_id, new_pairing).await
}

pub async fn get_pairings_by_subdomain_service(
    pool: &PgPool,
    restaurant_name: &String
)->Result<Vec<Pairing>, sqlx::Error>{
    get_pairing_by_subdomain_query(pool, restaurant_name).await
}

pub async fn get_pairings_by_session_service(
    pool: &PgPool,
    restaurant_id: i32,
)->Result<Vec<Pairing>, sqlx::Error>{
    get_pairing_by_session_query(pool, restaurant_id).await
}
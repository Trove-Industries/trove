use sqlx::PgPool;

use std::result::Result;
use crate::db::currency_queries::currency::{create_currency_query, get_all_currencies_query, get_currency_by_iso_query};
use crate::models::currency_models::currency::{Currency, NewCurrency};

/// Create a new currency
pub async fn create_currency_service(
    pool: &PgPool,
    new_currency: NewCurrency,
) -> Result<Currency, String> {
    create_currency_query(pool, new_currency)
        .await
        .map_err(|e| {
            eprintln!("❌ Error creating currency: {}", e);
            e.to_string()
        })
}

/// Get a currency by ISO code (e.g. "KES")
pub async fn get_currency_by_iso_service(
    pool: &PgPool,
    iso: &str,
) -> Result<Option<Currency>, String> {
    get_currency_by_iso_query(pool, iso)
        .await
        .map_err(|e| {
            eprintln!("❌ Error fetching currency by ISO '{}': {}", iso, e);
            e.to_string()
        })
}

/// Get all available currencies
pub async fn get_all_currencies_service(
    pool: &PgPool,
) -> Result<Vec<Currency>, String> {
    get_all_currencies_query(pool)
        .await
        .map_err(|e| {
            eprintln!("❌ Error fetching all currencies: {}", e);
            e.to_string()
        })
}

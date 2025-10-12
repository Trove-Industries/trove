use sqlx::{Error, PgPool};
use crate::models::currency_models::currency::{Currency, NewCurrency};

pub async fn create_currency_query(
    pool: &PgPool,
    currency_details: NewCurrency,
) -> Result<Currency, Error> {
    let new_currency = sqlx::query_as::<_, Currency>(
        r#"
        INSERT INTO currency (
            currency_name,
            currency_iso,
            currency_symbol,
            currency_rate
        )
        VALUES ($1, $2, $3, $4)
        RETURNING
            id,
            currency_name,
            currency_iso,
            currency_symbol,
            currency_rate,
            created_at,
            updated_at
        "#
    )
        .bind(currency_details.currency_name)
        .bind(currency_details.currency_iso)
        .bind(currency_details.currency_symbol)
        .bind(currency_details.currency_rate)
        .persistent(false)
        .fetch_one(pool)
        .await?;

    Ok(new_currency)
}

pub async fn get_currency_by_iso_query(
    pool: &PgPool,
    iso: &str,
) -> Result<Option<Currency>, Error> {
    let currency = sqlx::query_as::<_, Currency>(
        r#"
        SELECT
            id,
            currency_name,
            currency_iso,
            currency_symbol,
            currency_rate,
            created_at,
            updated_at
        FROM currency
        WHERE currency_iso = $1
        LIMIT 1
        "#
    )
        .bind(iso)
        .persistent(false)
        .fetch_optional(pool)
        .await?;

    Ok(currency)
}

pub async fn get_all_currencies_query(pool: &PgPool) -> Result<Vec<Currency>, Error> {
    let currencies = sqlx::query_as::<_, Currency>(
        r#"
        SELECT
            id,
            currency_name,
            currency_iso,
            currency_symbol,
            currency_rate,
            created_at,
            updated_at
        FROM currency
        ORDER BY currency_name ASC
        "#
    )
        .persistent(false)
        .fetch_all(pool)
        .await?;

    Ok(currencies)
}

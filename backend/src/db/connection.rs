use std::time::Duration;
use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn connection_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(25)  
        .min_connections(5)
        .acquire_timeout(Duration::from_secs(10))
        .idle_timeout(Duration::from_secs(300))
        .max_lifetime(Duration::from_secs(1800))
        // Important for pgbouncer transaction mode
        .test_before_acquire(false)
        .connect(&database_url)
        .await?;

    Ok(pool)
}
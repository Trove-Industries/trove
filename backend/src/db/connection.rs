use std::time::Duration;
use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn connection_pool(database_url: &str) ->Result<PgPool, sqlx::Error>{
    let pool = PgPoolOptions::new()
        .max_connections(40)
        .min_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&database_url)
        .await?;
    
    Ok(pool)
}
use sqlx::PgPool;

pub async fn get_restaurant_name_from_domain(
    pool: &PgPool,
    subdomain: &str,
) -> Result<Option<String>, sqlx::Error> {
    let result = sqlx::query_scalar::<_, String>(
        r#"
        SELECT restaurant_name
        FROM restaurants
        WHERE restaurant_subdomain = $1
        "#,
    )
        .bind(subdomain)
        .fetch_optional(pool)
        .await?;

    Ok(result)
}

use sqlx::{Error, PgPool};
use sqlx::types::Uuid;
use crate::models::user_models::user::{InitUser, User};

pub async fn create_init_user(
    pool: &PgPool,
    init_user: InitUser,
) -> Result<User, Error> {
    let trial_user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (username, is_verified, created_at, updated_at)
        VALUES ($1, FALSE, NOW(), NOW())
        RETURNING id, supabase_uid, email, username, is_verified, created_at, updated_at, last_seen_at
        "#
    )
        .bind(&init_user.username)
        .persistent(false)
        .fetch_one(pool)
        .await?;

    Ok(trial_user)
}

pub async fn verify_user(
    pool: &PgPool,
    restaurant_id: i32,
    supabase_uid: Uuid,
    email: &str,
) -> Result<User, Error> {
    let user_id: i32 = sqlx::query_scalar(
        "SELECT user_id FROM restaurants WHERE id = $1"
    )
        .bind(restaurant_id)
        .persistent(false)
        .fetch_one(pool)
        .await?;

    let verified_user = sqlx::query_as::<_, User>(
        r#"
        UPDATE users
        SET supabase_uid = $1,
            email = $2,
            is_verified = TRUE,
            updated_at = NOW()
        WHERE id = $3
        RETURNING id, supabase_uid, email, username, is_verified, created_at, updated_at, last_seen_at
        "#
    )
        .bind(supabase_uid)
        .bind(email)
        .bind(user_id)
        .persistent(false)
        .fetch_one(pool)
        .await?;

    Ok(verified_user)
}


pub async fn cleanup_expired_users(
    pool: &PgPool
) -> Result<u64, Error> {
    let result = sqlx::query(
        r#"
                DELETE FROM users WHERE is_verified = FALSE AND expires_at < NOW()
             "#
    )
        .persistent(false)
        .execute(pool)
        .await?;

    Ok(result.rows_affected())
}
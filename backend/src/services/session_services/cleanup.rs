use chrono::Utc;
use sqlx::PgPool;
use std::time::Duration;
use tracing::{info, error};

pub async fn start_session_cleanup_task(pool: PgPool) {
    tokio::spawn(async move {
        loop {
            // Sleep 24 hours between runs
            tokio::time::sleep(Duration::from_secs(60 * 60 * 24)).await;

            if let Err(e) = cleanup_expired_sessions(&pool).await {
                error!("❌ Session cleanup failed: {:?}", e);
            }
        }
    });
}

async fn cleanup_expired_sessions(pool: &PgPool) -> Result<(), sqlx::Error> {
    let now = Utc::now();

    // 1️⃣ Mark expired sessions inactive
    let inactive_count = sqlx::query!(
        r#"UPDATE sessions SET is_active = FALSE WHERE expires_at < $1 AND is_active = TRUE"#,
        now
    )
        .execute(pool)
        .await?
        .rows_affected();

    // 2️⃣ Delete inactive sessions older than 30 days
    let deleted_count = sqlx::query!(
        r#"DELETE FROM sessions WHERE is_active = FALSE AND expires_at < $1"#,
        now - chrono::Duration::days(30)
    )
        .execute(pool)
        .await?
        .rows_affected();

    info!("🧹 Session cleanup: marked {} inactive, deleted {}", inactive_count, deleted_count);
    Ok(())
}

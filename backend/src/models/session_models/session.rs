use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Session {
    pub id: i32,
    pub user_id: i32,
    pub session_token: String,
    pub created_at: chrono::DateTime<Utc>,
    pub expires_at: chrono::DateTime<Utc>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub is_active: bool,
}

#[derive(Debug)]
pub struct NewSession {
    pub user_id: i32,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

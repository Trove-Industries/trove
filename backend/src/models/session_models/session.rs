use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct Session {
    pub id: i32,
    pub user_id: i32,
    pub session_token: Uuid,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewSession {
    pub user_id: i32,
    pub session_token: Uuid,
    pub expires_at: DateTime<Utc>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

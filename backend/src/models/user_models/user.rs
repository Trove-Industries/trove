use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::Uuid;

#[derive(Debug,Serialize,Deserialize,FromRow)]
pub struct User {
    pub id: i32,
    pub supabase_uid: Option<Uuid>,
    pub email: Option<String>,
    pub username: String,
    pub is_verified: bool,
}

#[derive(Debug,Serialize,Deserialize,FromRow)]
pub struct InitUser{
    pub username: String
}




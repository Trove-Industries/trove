use chrono::{NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub supabase_uid: Option<Uuid>,
    pub email: Option<String>,
    pub username: String,
    pub is_verified: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub last_seen_at: NaiveDateTime
}

#[derive(Debug,Serialize,Deserialize,FromRow)]
pub struct InitUser{
    pub username: String
}

pub struct UpdateUser{
    
}




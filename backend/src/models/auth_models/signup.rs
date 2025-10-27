use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct SignupResponse {
    pub message: String,
    pub user_id: Option<String>,
}

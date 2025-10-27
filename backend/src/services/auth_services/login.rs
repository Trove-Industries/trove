use crate::utils::supabase_client::SupabaseClient;
use sqlx::PgPool;
use crate::models::auth_models::login::{LoginRequest, LoginResponse};
use uuid::Uuid;
use serde_json::Value;

pub async fn login_service(
    _pool: &PgPool,
    supabase: &SupabaseClient,
    req: LoginRequest,
) -> Result<LoginResponse, String> {
    // Call Supabase
    let response: Value = supabase
        .login_user(&req.email, &req.password)
        .await
        .map_err(|e| format!("Supabase login failed: {}", e))?;

    // Extract user object
    let user = response
        .get("user")
        .ok_or_else(|| "Missing 'user' in Supabase response".to_string())?;

    // Extract user ID (UUID)
    let uid_str = user
        .get("id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "Missing user id in Supabase response".to_string())?;

    // Validate UUID (optional)
    let _supabase_uid = Uuid::parse_str(uid_str)
        .map_err(|e| format!("Invalid UUID from Supabase: {}", e))?;

    // Extract session tokens from "session" object if present
    let session = response.get("session");

    let access_token = session
        .and_then(|s| s.get("access_token"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let refresh_token = session
        .and_then(|s| s.get("refresh_token"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let expires_at = session
        .and_then(|s| s.get("expires_at"))
        .and_then(|v| {
            // could be iso or epoch; convert to string as-is
            v.as_str().map(|s| s.to_string()).or_else(|| Some(v.to_string()))
        });

    Ok(LoginResponse {
        message: "Login successful".to_string(),
        user_id: Some(uid_str.to_string()),
        access_token,
        refresh_token,
        expires_at,
    })
}

use sqlx::PgPool;
use uuid::Uuid;
use crate::db::user::user_queries::verify_user;
use crate::models::auth_models::signup::{SignupRequest, SignupResponse};
use crate::utils::supabase_client::SupabaseClient;

pub async fn signup_service(
    pool: &PgPool,
    supabase: &SupabaseClient,
    req: SignupRequest,
    restaurant_id: i32,
) -> Result<SignupResponse, String> {
    // Signup in Supabase
    let response = supabase
        .signup_user(&req.email, &req.password)
        .await
        .map_err(|e| e.to_string())?;

    // Extract Supabase UID
    let supabase_uid_str = response.get("user")
        .and_then(|u| u.get("id"))
        .and_then(|v| v.as_str())
        .ok_or_else(|| "Supabase UID not returned".to_string())?;

    let supabase_uid = Uuid::parse_str(supabase_uid_str)
        .map_err(|e| format!("Invalid UUID from Supabase: {}", e))?;

    // Update local DB with Supabase UID, email, and mark verified
    let verified_user = verify_user(pool, restaurant_id, supabase_uid, &req.email)
        .await
        .map_err(|e| format!("Failed to verify user: {}", e))?;

    Ok(SignupResponse {
        message: "User signed up successfully".to_string(),
        user_id: verified_user.supabase_uid.map(|uid| uid.to_string())
    })
}

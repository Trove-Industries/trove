use sqlx::PgPool;
use crate::utils::supabase_client::SupabaseClient;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub supabase: SupabaseClient,
}

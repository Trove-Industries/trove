use sqlx::PgPool;
use crate::admin::db::admin_users_queries::get_all_users_query;
use crate::models::user_models::user::User;

pub async fn get_all_users_service(
    pool: &PgPool
)->Result<Vec<User>, sqlx::Error>{
    get_all_users_query(pool).await
}
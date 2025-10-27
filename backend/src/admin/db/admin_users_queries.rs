use sqlx::{Error, PgPool};
use crate::models::user_models::user::User;

pub async  fn get_all_users_query(
    pool: &PgPool,
)->Result<Vec<User>, Error>{

    let user = sqlx::query_as::<_,User>(
        r#"
                SELECT * FROM users
            "#
    )
        .persistent(false)
        .fetch_all(pool)
        .await?;

    Ok(user)
}
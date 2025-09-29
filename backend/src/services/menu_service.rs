use sqlx::PgPool;
use crate::db::menu_queries;
use crate::db::menu_queries::{insert_menu, get_menu_by_restaurant};
use crate::models::menu_item::{MenuItem, NewMenuItem};

pub async fn create_menu(
    pool: &PgPool,
    new_item: NewMenuItem,
) ->Result<MenuItem, sqlx::Error>{
    menu_queries::insert_menu(pool ,new_item).await
}

pub async fn get_menu(
    pool: &PgPool,
    restaurant_id: i32
) ->Result<Vec<MenuItem>, sqlx::Error>{
    menu_queries::get_menu_by_restaurant(pool, restaurant_id).await
}

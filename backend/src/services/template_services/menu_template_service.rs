use sqlx::PgPool;
use crate::db::template_queries::menu_template_queries::{get_menu_data, get_restaurant_data};
use crate::models::template_models::menu_template_models::{FullData};

pub async fn get_restaurant_with_menu(
    pool: &PgPool,
    subdomain: &String
) ->Result<FullData, sqlx::Error> {
    let restaurant = get_restaurant_data(pool, subdomain).await?;

    let menu_items = get_menu_data(pool, restaurant.id).await?;

    Ok(
        FullData{
            restaurant,
            menu_items,
        }
    )
}

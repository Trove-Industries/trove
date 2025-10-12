use sqlx::PgPool;
use crate::db::menu_queries::categories::get_categories_query;
use crate::db::menu_queries::ingredients::get_ingredient_query;
use crate::db::menu_queries::meal_groups::get_meal_group_query;
use crate::db::menu_queries::meals::get_meal_query;
use crate::db::menu_queries::pairings::get_pairing_query;
use crate::db::menu_queries::sizes::get_size_query;
use crate::db::restaurants_queries::restaurants::get_restaurant_query;
use crate::models::template_models::menu_template_model::FullMenuData;


pub async fn get_full_menu_data(pool: &PgPool, restaurant_name: &String) -> FullMenuData {
    let restaurant = get_restaurant_query(pool, restaurant_name).await.unwrap();
    let categories = get_categories_query(pool, restaurant_name).await.unwrap();
    let meal_groups = get_meal_group_query(pool, restaurant_name).await.unwrap();
    let meals = get_meal_query(pool, restaurant_name).await.unwrap();
    let sizes = get_size_query(pool, restaurant_name).await.unwrap();
    let pairings = get_pairing_query(pool, restaurant_name).await.unwrap();
    let ingredients = get_ingredient_query(pool, restaurant_name).await.unwrap();

    FullMenuData {
        restaurant,
        categories,
        meal_groups,
        meals,
        sizes,
        pairings,
        ingredients,
    }
}

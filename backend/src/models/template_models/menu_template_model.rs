use serde::Serialize;
use crate::models::menu_models::categories::Category;
use crate::models::menu_models::ingredient::Ingredient;
use crate::models::menu_models::meal_groups::MealGroup;
use crate::models::menu_models::meals::Meal;
use crate::models::menu_models::pairings::Pairing;
use crate::models::menu_models::sizes::Size;
use crate::models::restaurants_models::restaurants::Restaurant;

#[derive(Serialize)]
pub struct FullMenuData {
    pub restaurant: Vec<Restaurant>,
    pub categories: Vec<Category>,
    pub meal_groups: Vec<MealGroup>,
    pub meals: Vec<Meal>,
    pub sizes: Vec<Size>,
    pub pairings: Vec<Pairing>,
    pub ingredients: Vec<Ingredient>,
}

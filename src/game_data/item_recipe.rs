use crate::game_data::ItemId;
use crate::simulation::prelude::Milliseconds;

pub type RecipeId = u32;

pub const RECIPE_A_ID: RecipeId = 1;
pub const RECIPE_B_ID: RecipeId = 2;
pub const RECIPE_C_ID: RecipeId = 3;

pub struct ItemRecipe {
    /// Unique ID to differentiate between recipes
    pub id: RecipeId,
    /// Useful to differentiate if an item has multiple recipes
    pub name: String,
    /// How long it will take to process this recipe once, in milliseconds
    pub duration: Milliseconds,
    /// The required ingredients to get production starting
    pub input: Vec<ItemRecipeElement>,
    /// Yields of a single production run
    pub output: Vec<ItemRecipeElement>,
}

pub struct ItemRecipeElement {
    pub item_id: ItemId,
    pub amount: u32,
}

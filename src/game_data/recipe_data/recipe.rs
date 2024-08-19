use crate::game_data::recipe_data::RecipeId;
use crate::game_data::ItemId;
use crate::simulation::prelude::Milliseconds;
use serde::Deserialize;

/// Defines a single production step.
#[derive(Deserialize)]
pub struct RecipeData {
    /// Unique ID to differentiate between recipes
    pub id: RecipeId,
    /// Useful to differentiate if an item has multiple recipes
    pub name: String,
    /// How long it will take to process this recipe once, in milliseconds
    pub duration: Milliseconds,
    /// The required ingredients to get production starting
    pub input: Vec<RecipeElement>,
    /// Yields of a single production run
    pub output: Vec<RecipeElement>,
}

#[derive(Deserialize)]
pub struct RecipeElement {
    pub item_id: ItemId,
    pub amount: u32,
}

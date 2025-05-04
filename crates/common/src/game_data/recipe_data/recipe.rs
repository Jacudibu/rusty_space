use crate::game_data::ItemId;
use crate::game_data::generic_manifest_without_raw_data::DataCanBeUsedAsRawData;
use crate::game_data::recipe_data::RecipeId;
use crate::simulation_time::Milliseconds;
use bevy::prelude::TypePath;
use serde::Deserialize;

/// Defines a single production step.
#[derive(TypePath, Deserialize)]
#[allow(dead_code)]
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

impl DataCanBeUsedAsRawData for RecipeData {}

/// Defines an item with a given quantity.
#[derive(Deserialize, Copy, Clone)]
pub struct RecipeElement {
    /// The item represented by this object
    pub item_id: ItemId,
    /// The quantity of the assigned item.
    pub amount: u32,
}

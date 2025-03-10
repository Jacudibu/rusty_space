use crate::game_data::generic_manifest_without_raw_data::DataCanBeUsedAsRawData;
use crate::game_data::recipe_data::RecipeId;
use crate::game_data::ItemId;
use crate::simulation::prelude::Milliseconds;
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

#[derive(Deserialize, Copy, Clone)]
pub struct RecipeElement {
    pub item_id: ItemId,
    pub amount: u32,
}

use crate::game_data::generic_manifest_without_raw_data::DataCanBeUsedAsRawData;
use crate::game_data::production_module_data::ProductionModuleId;
use crate::game_data::RecipeId;
use bevy::prelude::TypePath;
use serde::Deserialize;

/// Defines the costs and capabilities of a single Production Line
#[derive(TypePath, Deserialize)]
#[allow(dead_code)]
pub struct ProductionModuleData {
    /// Unique ID to differentiate between recipes
    pub id: ProductionModuleId,
    /// User Facing name thingy
    pub name: String,
    /// List of things that can be produced
    pub available_recipes: Vec<RecipeId>,
}

impl DataCanBeUsedAsRawData for ProductionModuleData {}

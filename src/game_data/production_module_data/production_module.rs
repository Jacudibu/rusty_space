use crate::game_data::generic_manifest_without_raw_data::DataCanBeUsedAsRawData;
use crate::game_data::production_module_data::ProductionModuleId;
use crate::game_data::{RecipeElement, RecipeId};
use bevy::prelude::TypePath;
use serde::Deserialize;

/// Defines the costs and capabilities of a single Production Line
#[derive(TypePath, Deserialize)]
pub struct ProductionModuleData {
    /// Unique ID to differentiate between recipes
    pub id: ProductionModuleId,
    /// User Facing name thingy
    pub name: String,
    /// List of things that can be produced
    pub available_recipes: Vec<RecipeId>,
    /// The amount of build power necessary to build this module.
    pub required_build_power: u32,
    /// The bill of materials required to build this module
    pub required_materials: Vec<RecipeElement>,
}

impl DataCanBeUsedAsRawData for ProductionModuleData {}

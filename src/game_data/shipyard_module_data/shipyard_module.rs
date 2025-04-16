use crate::game_data::generic_manifest_without_raw_data::DataCanBeUsedAsRawData;
use crate::game_data::{RecipeElement, ShipyardModuleId};
use bevy::prelude::TypePath;
use serde::Deserialize;

/// Defines the costs and capabilities of a single ship production line
#[derive(TypePath, Deserialize)]
pub struct ShipyardModuleData {
    /// Unique ID to differentiate between recipes
    pub id: ShipyardModuleId,
    /// User Facing name thingy
    pub name: String,
    /// The amount of build power necessary to build this module.
    pub required_build_power: u32,
    /// The bill of materials required to build this module
    pub required_materials: Vec<RecipeElement>,
    // TODO: Settings to only allow certain ship types should be defined here, maybe with build speed modifiers.
}

impl DataCanBeUsedAsRawData for ShipyardModuleData {}

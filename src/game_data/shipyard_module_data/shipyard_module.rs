use crate::game_data::generic_manifest_without_raw_data::DataCanBeUsedAsRawData;
use crate::game_data::ShipyardModuleId;
use bevy::prelude::TypePath;
use serde::Deserialize;

/// Defines the costs and capabilities of a single ship production line
#[derive(TypePath, Deserialize)]
#[allow(dead_code)]
pub struct ShipyardModuleData {
    /// Unique ID to differentiate between recipes
    pub id: ShipyardModuleId,

    /// User Facing name thingy
    pub name: String,

    /// The amount of build power necessary to build this module.
    pub required_build_power: u32,
    // TODO: Settings to only allow certain ship types should be defined here, maybe with build speed modifiers.
}

impl DataCanBeUsedAsRawData for ShipyardModuleData {}

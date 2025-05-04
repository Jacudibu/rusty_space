use crate::game_data::generic_manifest_without_raw_data::DataCanBeUsedAsRawData;
use crate::game_data::{Constructable, ConstructableSiteData, ShipyardModuleId};
use bevy::prelude::TypePath;
use serde::Deserialize;

/// Defines the costs and capabilities of a single ship production line
#[derive(TypePath, Deserialize)]
pub struct ShipyardModuleData {
    /// Unique ID to differentiate between recipes
    pub id: ShipyardModuleId,
    /// User Facing name thingy
    pub name: String,
    /// Stuff required to construct this module.
    pub constructable_data: ConstructableSiteData,
    // TODO: Settings to only allow certain ship types should be defined here, maybe with build speed modifiers.
}

impl DataCanBeUsedAsRawData for ShipyardModuleData {}

impl Constructable for ShipyardModuleData {
    fn get_constructable_data(&self) -> &ConstructableSiteData {
        &self.constructable_data
    }
}

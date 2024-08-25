use crate::game_data::ShipyardModuleId;
use serde::Deserialize;

/// Defines the costs and capabilities of a single ship production line
#[derive(Deserialize)]
pub struct ShipyardModuleData {
    /// Unique ID to differentiate between recipes
    pub id: ShipyardModuleId,
    /// User Facing name thingy
    pub name: String,
    // TODO: Settings to only allow certain ship types should be defined here, maybe with build speed modifiers.
}

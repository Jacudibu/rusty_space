use crate::game_data::ShipyardModuleId;
use serde::Deserialize;

/// Defines the costs and capabilities of a single ship production line
#[derive(Deserialize)]
pub struct ShipyardModuleDefinition {
    /// Unique ID to differentiate between recipes
    pub id: ShipyardModuleId,
    /// User Facing name thingy
    pub name: String,
    // TODO: Settings to only allow certain ship types should be defined here.
}

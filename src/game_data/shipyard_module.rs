pub type ShipyardModuleId = u32;

pub const SHIPYARD_MODULE_ID: ShipyardModuleId = 1;

/// Defines the costs and capabilities of a single ship production line
pub struct ShipyardModuleDefinition {
    /// Unique ID to differentiate between recipes
    pub id: ShipyardModuleId,
    /// User Facing name thingy
    pub name: String,
    // TODO: Settings to only allow certain ship types should be defined here.
}

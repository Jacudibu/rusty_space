use crate::game_data::ItemRecipeElement;
use crate::simulation::prelude::Milliseconds;

pub type ShipConfigId = u32;

pub const DEBUG_SHIP_CONFIG: ShipConfigId = 1;

/// Defines the individual parts from which a ship is built.
///
/// Multiple ships can share the same configuration through its ID field.
pub struct ShipConfiguration {
    pub id: ShipConfigId,
    pub name: String,
    pub duration: Milliseconds,
    pub materials: Vec<ItemRecipeElement>,
}

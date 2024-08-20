use crate::game_data::RecipeElement;
use crate::session_data::ShipConfigId;
use crate::simulation::prelude::Milliseconds;
use serde::Deserialize;

/// Defines the individual parts from which a ship is built.
///
/// Multiple ships can share the same configuration through their ID field.
#[derive(Deserialize)]
pub struct ShipConfiguration {
    pub id: ShipConfigId,
    pub name: String,
    pub duration: Milliseconds,
    pub materials: Vec<RecipeElement>,
}

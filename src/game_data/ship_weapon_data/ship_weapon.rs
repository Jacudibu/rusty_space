use crate::game_data::RecipeElement;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ShipWeaponData {
    pub energy_cost: u16,
    pub cpu_cost: u16,

    pub ore_mining_strength: Option<u16>,
    pub gas_harvesting_strength: Option<u16>,

    /// Bill of materials required to build this.
    pub required_materials: Vec<RecipeElement>,
}

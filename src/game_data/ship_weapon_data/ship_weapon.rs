use crate::game_data::RecipeElement;
use crate::game_data::generic_manifest_without_raw_data::DataCanBeUsedAsRawData;
use bevy::prelude::TypePath;
use serde::Deserialize;

#[derive(TypePath, Deserialize)]
pub struct ShipWeaponData {
    pub energy_cost: u16,
    pub cpu_cost: u16,

    /// How much build power this... 'weapon' provides, if any.
    pub build_power: Option<u32>,

    /// How much ore per second this weapon can mine, if any.
    pub asteroid_mining_strength: Option<u32>,

    /// How much gas per second this weapon can harvest, if any.
    pub gas_harvesting_strength: Option<u32>,

    /// Bill of materials required to build this.
    pub required_materials: Vec<RecipeElement>,
}

impl DataCanBeUsedAsRawData for ShipWeaponData {}

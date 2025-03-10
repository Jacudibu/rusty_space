use crate::game_data::from_mock_data::FromMockData;
use crate::game_data::generic_manifest_without_raw_data::GenericManifestWithoutRawData;
use crate::game_data::ship_weapon_data::ship_weapon::ShipWeaponData;
use crate::game_data::ship_weapon_data::{
    CONSTRUCTION_TOOL_ID, GAS_COLLECTOR_ID, ORE_MINING_LASER_ID,
};
use crate::game_data::{REFINED_METALS_ITEM_ID, RecipeElement, SILICA_ITEM_ID};
use bevy::prelude::World;
use bevy::utils::HashMap;

/// Contains all parsed Ship Weapon Modules.
pub type ShipWeaponManifest = GenericManifestWithoutRawData<ShipWeaponData>;

impl FromMockData for ShipWeaponManifest {
    fn from_mock_data(_world: &mut World) -> Self {
        let mut equipment = HashMap::new();

        equipment.insert(
            CONSTRUCTION_TOOL_ID,
            ShipWeaponData {
                energy_cost: 5,
                cpu_cost: 5,
                build_power: Some(10),
                gas_harvesting_strength: None,
                asteroid_mining_strength: None,
                required_materials: vec![RecipeElement {
                    item_id: REFINED_METALS_ITEM_ID,
                    amount: 5,
                }],
            },
        );
        equipment.insert(
            ORE_MINING_LASER_ID,
            ShipWeaponData {
                energy_cost: 5,
                cpu_cost: 5,
                build_power: None,
                gas_harvesting_strength: None,
                asteroid_mining_strength: Some(10),
                required_materials: vec![RecipeElement {
                    item_id: REFINED_METALS_ITEM_ID,
                    amount: 5,
                }],
            },
        );
        equipment.insert(
            GAS_COLLECTOR_ID,
            ShipWeaponData {
                energy_cost: 5,
                cpu_cost: 5,
                build_power: None,
                gas_harvesting_strength: Some(10),
                asteroid_mining_strength: None,
                required_materials: vec![RecipeElement {
                    item_id: SILICA_ITEM_ID,
                    amount: 5,
                }],
            },
        );

        Self::from(equipment)
    }
}

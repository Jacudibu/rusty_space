use crate::game_data::from_mock_data::FromMockData;
use crate::game_data::ship_weapon_data::ship_weapon::ShipWeaponData;
use crate::game_data::ship_weapon_data::{
    ShipWeaponId, MOCK_SHIP_WEAPON_GAS_COLLECTOR_ID, MOCK_SHIP_WEAPON_ORE_MINING_LASER_ID,
};
use crate::game_data::{RecipeElement, MOCK_ITEM_ID_A, MOCK_ITEM_ID_B};
use bevy::prelude::{Asset, Resource, TypePath, World};
use bevy::utils::HashMap;
use leafwing_manifest::identifier::Id;
use leafwing_manifest::manifest::{Manifest, ManifestFormat};
use serde::Deserialize;

#[derive(Resource, Asset, TypePath, Deserialize)]
pub struct ShipWeaponManifest {
    equipment: HashMap<ShipWeaponId, ShipWeaponData>,
}

impl ShipWeaponManifest {
    #[inline]
    #[must_use]
    pub fn get_by_ref(&self, id: &ShipWeaponId) -> Option<&ShipWeaponData> {
        self.equipment.get(id)
    }
}

impl FromMockData for ShipWeaponManifest {
    fn from_mock_data(world: &mut World) -> Self {
        let mut equipment = HashMap::new();

        equipment.insert(
            MOCK_SHIP_WEAPON_ORE_MINING_LASER_ID,
            ShipWeaponData {
                energy_cost: 5,
                cpu_cost: 5,
                gas_harvesting_strength: Some(10),
                asteroid_mining_strength: None,
                required_materials: vec![RecipeElement {
                    item_id: MOCK_ITEM_ID_A,
                    amount: 5,
                }],
            },
        );
        equipment.insert(
            MOCK_SHIP_WEAPON_GAS_COLLECTOR_ID,
            ShipWeaponData {
                energy_cost: 5,
                cpu_cost: 5,
                gas_harvesting_strength: Some(10),
                asteroid_mining_strength: None,
                required_materials: vec![RecipeElement {
                    item_id: MOCK_ITEM_ID_B,
                    amount: 5,
                }],
            },
        );

        Self::from_raw_manifest(ShipWeaponManifest { equipment }, world).unwrap()
    }
}

impl Manifest for ShipWeaponManifest {
    type RawManifest = ShipWeaponManifest;
    type RawItem = ShipWeaponData;
    type Item = ShipWeaponData;
    type ConversionError = std::convert::Infallible;
    const FORMAT: ManifestFormat = ManifestFormat::Custom;

    fn from_raw_manifest(
        raw_manifest: Self::RawManifest,
        _world: &mut World,
    ) -> Result<Self, Self::ConversionError> {
        Ok(Self {
            equipment: raw_manifest.equipment,
        })
    }

    fn get(&self, id: Id<Self::Item>) -> Option<&Self::Item> {
        self.get_by_ref(&id)
    }
}

use crate::game_data::from_mock_data::FromMockData;
use crate::game_data::generic_manifest_without_raw_data::GenericManifestWithoutRawData;
use crate::game_data::ship_hull_data::ship_hull::ShipManeuverability;
use crate::game_data::ship_hull_data::{ShipHullData, MOCK_SHIP_HULL_A_ID, MOCK_SHIP_HULL_A_NAME};
use crate::game_data::{RecipeElement, MOCK_ITEM_ID_A, MOCK_ITEM_ID_B, MOCK_ITEM_ID_C};
use crate::utils::ShipSize;
use bevy::prelude::World;
use bevy::utils::HashMap;

/// Contains all parsed Ship Hull Modules.
pub type ShipHullManifest = GenericManifestWithoutRawData<ShipHullData>;

impl FromMockData for ShipHullManifest {
    fn from_mock_data(_world: &mut World) -> Self {
        let mut mock_hulls = HashMap::new();

        mock_hulls.insert(
            MOCK_SHIP_HULL_A_ID,
            ShipHullData {
                id: MOCK_SHIP_HULL_A_ID,
                name: MOCK_SHIP_HULL_A_NAME.into(),
                ship_size: ShipSize::S,
                weapon_slots: 2,
                inventory_size: 100,
                build_time: 5000,
                maneuverability: ShipManeuverability {
                    max_speed: 100.0,
                    acceleration: 10.0,
                    deceleration: 30.0,
                    max_angular_speed: 1.0,
                    angular_acceleration: 1.0,
                },
                required_materials: vec![
                    RecipeElement {
                        item_id: MOCK_ITEM_ID_A,
                        amount: 50,
                    },
                    RecipeElement {
                        item_id: MOCK_ITEM_ID_B,
                        amount: 23,
                    },
                    RecipeElement {
                        item_id: MOCK_ITEM_ID_C,
                        amount: 74,
                    },
                ],
            },
        );

        Self::from(mock_hulls)
    }
}

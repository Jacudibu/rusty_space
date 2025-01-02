use crate::game_data::ship_hull_data::raw_ship_hull::{RawShipHullData, ShipManeuverability};
use crate::game_data::ship_hull_data::{SHIP_HULL_MINER_NAME, SHIP_HULL_TRANSPORT_NAME};
use crate::game_data::{RecipeElement, MOCK_ITEM_A_ID, MOCK_ITEM_B_ID, MOCK_ITEM_C_ID};
use crate::utils::ShipSize;
use bevy::asset::Asset;
use bevy::prelude::TypePath;
use serde::Deserialize;

#[derive(Asset, TypePath, Deserialize)]
pub struct RawShipHullManifest {
    pub(crate) raw_data: Vec<RawShipHullData>,
}

impl RawShipHullManifest {
    pub fn mock_data() -> Self {
        Self {
            raw_data: vec![
                RawShipHullData {
                    name: SHIP_HULL_TRANSPORT_NAME.into(),
                    sprite: "ship_transport.png".into(),
                    ship_size: ShipSize::S,
                    weapon_slots: 0,
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
                            item_id: MOCK_ITEM_A_ID,
                            amount: 50,
                        },
                        RecipeElement {
                            item_id: MOCK_ITEM_B_ID,
                            amount: 23,
                        },
                        RecipeElement {
                            item_id: MOCK_ITEM_C_ID,
                            amount: 74,
                        },
                    ],
                },
                RawShipHullData {
                    name: SHIP_HULL_MINER_NAME.into(),
                    sprite: "ship.png".into(),
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
                            item_id: MOCK_ITEM_A_ID,
                            amount: 50,
                        },
                        RecipeElement {
                            item_id: MOCK_ITEM_B_ID,
                            amount: 23,
                        },
                        RecipeElement {
                            item_id: MOCK_ITEM_C_ID,
                            amount: 74,
                        },
                    ],
                },
            ],
        }
    }
}

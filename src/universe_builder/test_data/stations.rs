use crate::game_data::{
    DEBUG_ITEM_ID_A, DEBUG_ITEM_ID_B, DEBUG_ITEM_ID_C, PRODUCTION_MODULE_A_ID,
    PRODUCTION_MODULE_B_ID, PRODUCTION_MODULE_C_ID, RECIPE_A_ID, RECIPE_B_ID, RECIPE_C_ID,
    SHIPYARD_MODULE_ID,
};
use crate::universe_builder::local_hex_position::LocalHexPosition;
use crate::universe_builder::station_builder::StationSpawnData;
use crate::universe_builder::test_data::coordinates::{BOTTOM_LEFT, CENTER};
use bevy::prelude::Vec2;

pub fn create_test_data() -> StationSpawnData {
    let mut result = StationSpawnData::default();
    result
        .add(
            LocalHexPosition::new(BOTTOM_LEFT, Vec2::new(-200.0, -200.0)),
            "Station A".into(),
        )
        .with_production(5, PRODUCTION_MODULE_B_ID, RECIPE_B_ID)
        .with_buys(vec![DEBUG_ITEM_ID_A])
        .with_sells(vec![DEBUG_ITEM_ID_B]);

    result
        .add(
            LocalHexPosition::new(CENTER, Vec2::new(200.0, -200.0)),
            "Station B".into(),
        )
        .with_production(3, PRODUCTION_MODULE_C_ID, RECIPE_C_ID)
        .with_buys(vec![DEBUG_ITEM_ID_B])
        .with_sells(vec![DEBUG_ITEM_ID_C]);

    result
        .add(
            LocalHexPosition::new(CENTER, Vec2::new(0.0, 200.0)),
            "Station C".into(),
        )
        .with_production(1, PRODUCTION_MODULE_A_ID, RECIPE_A_ID)
        .with_buys(vec![DEBUG_ITEM_ID_C])
        .with_sells(vec![DEBUG_ITEM_ID_A]);

    result
        .add(
            LocalHexPosition::new(CENTER, Vec2::new(0.0, 0.0)),
            "Shipyard".into(),
        )
        .with_shipyard(2, SHIPYARD_MODULE_ID)
        .with_buys(vec![DEBUG_ITEM_ID_A, DEBUG_ITEM_ID_B, DEBUG_ITEM_ID_C]);

    result
}

use crate::game_data::{
    MOCK_ITEM_ID_A, MOCK_ITEM_ID_B, MOCK_ITEM_ID_C, MOCK_PRODUCTION_MODULE_A_ID,
    MOCK_PRODUCTION_MODULE_B_ID, MOCK_PRODUCTION_MODULE_C_ID, MOCK_RECIPE_A_ID, MOCK_RECIPE_B_ID,
    MOCK_RECIPE_C_ID, MOCK_SHIPYARD_MODULE_ID,
};
use crate::persistence::local_hex_position::LocalHexPosition;
use crate::persistence::test_universe::coordinates::{BOTTOM_LEFT, CENTER};
use crate::persistence::{SaveDataCollection, StationSaveData};
use bevy::prelude::Vec2;

pub fn create_test_data() -> SaveDataCollection<StationSaveData> {
    let mut result = SaveDataCollection::<StationSaveData>::default();

    result
        .add(
            LocalHexPosition::new(CENTER, Vec2::new(0.0, 200.0)),
            "Station A".into(),
        )
        .with_production(1, MOCK_PRODUCTION_MODULE_A_ID, MOCK_RECIPE_A_ID)
        .with_buys(vec![MOCK_ITEM_ID_C])
        .with_sells(vec![MOCK_ITEM_ID_A]);

    result
        .add(
            LocalHexPosition::new(BOTTOM_LEFT, Vec2::new(-200.0, -200.0)),
            "Station B".into(),
        )
        .with_production(5, MOCK_PRODUCTION_MODULE_B_ID, MOCK_RECIPE_B_ID)
        .with_buys(vec![MOCK_ITEM_ID_A])
        .with_sells(vec![MOCK_ITEM_ID_B]);

    result
        .add(
            LocalHexPosition::new(CENTER, Vec2::new(200.0, -200.0)),
            "Station C".into(),
        )
        .with_production(3, MOCK_PRODUCTION_MODULE_C_ID, MOCK_RECIPE_C_ID)
        .with_buys(vec![MOCK_ITEM_ID_B])
        .with_sells(vec![MOCK_ITEM_ID_C]);

    result
        .add(
            LocalHexPosition::new(CENTER, Vec2::new(0.0, 0.0)),
            "Shipyard".into(),
        )
        .with_shipyard(2, MOCK_SHIPYARD_MODULE_ID)
        .with_buys(vec![MOCK_ITEM_ID_A, MOCK_ITEM_ID_B, MOCK_ITEM_ID_C]);

    result
}

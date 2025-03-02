use crate::game_data::{
    CRYSTAL_ORE_ITEM_ID, HYDROGEN_ITEM_ID, IRON_ORE_ITEM_ID, MOCK_SHIPYARD_MODULE_ID,
    REFINED_METALS_ITEM_ID, REFINED_METALS_PRODUCTION_MODULE_ID, REFINED_METALS_RECIPE_ID,
    SILICA_ITEM_ID, SILICA_PRODUCTION_MODULE_ID, SILICA_RECIPE_ID, WAFERS_PRODUCTION_MODULE_ID,
    WAFERS_RECIPE_ID, WAFER_ITEM_ID,
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
            "Crystal Processor".into(),
        )
        .with_production(1, SILICA_PRODUCTION_MODULE_ID, SILICA_RECIPE_ID)
        .with_buys(vec![CRYSTAL_ORE_ITEM_ID])
        .with_sells(vec![SILICA_ITEM_ID]);

    result
        .add(
            LocalHexPosition::new(BOTTOM_LEFT, Vec2::new(-200.0, -200.0)),
            "Forge".into(),
        )
        .with_production(
            5,
            REFINED_METALS_PRODUCTION_MODULE_ID,
            REFINED_METALS_RECIPE_ID,
        )
        .with_buys(vec![IRON_ORE_ITEM_ID])
        .with_sells(vec![REFINED_METALS_ITEM_ID]);

    result
        .add(
            LocalHexPosition::new(CENTER, Vec2::new(200.0, -200.0)),
            "Wafer Fab".into(),
        )
        .with_production(3, WAFERS_PRODUCTION_MODULE_ID, WAFERS_RECIPE_ID)
        .with_buys(vec![SILICA_ITEM_ID, HYDROGEN_ITEM_ID])
        .with_sells(vec![WAFER_ITEM_ID]);

    result
        .add(
            LocalHexPosition::new(CENTER, Vec2::new(0.0, 0.0)),
            "Shipyard".into(),
        )
        .with_shipyard(2, MOCK_SHIPYARD_MODULE_ID)
        .with_buys(vec![REFINED_METALS_ITEM_ID, WAFER_ITEM_ID]);

    result
}

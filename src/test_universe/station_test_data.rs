use crate::test_universe::coordinates::{BOTTOM_LEFT, CENTER};
use bevy::prelude::Vec2;
use common::game_data::{
    CRYSTAL_ORE_ITEM_ID, ConstructableModuleId, HYDROGEN_ITEM_ID, IRON_ORE_ITEM_ID,
    MOCK_SHIPYARD_MODULE_ID, REFINED_METALS_ITEM_ID, REFINED_METALS_PRODUCTION_MODULE_ID,
    REFINED_METALS_RECIPE_ID, SILICA_ITEM_ID, SILICA_PRODUCTION_MODULE_ID, SILICA_RECIPE_ID,
    WAFER_ITEM_ID, WAFERS_PRODUCTION_MODULE_ID, WAFERS_RECIPE_ID,
};
use common::types::local_hex_position::LocalHexPosition;
use common::types::polar_coordinates::PolarCoordinates;
use persistence::data::{SaveDataCollection, StationSaveData};
use universe_builder::station_builder::StationBuilder;

pub fn create_test_data() -> SaveDataCollection<StationSaveData> {
    let mut result = StationBuilder::default();

    result
        .add(
            LocalHexPosition::from_polar(BOTTOM_LEFT, PolarCoordinates::new(200.0, 220.0)),
            "Forge".into(),
        )
        .with_production(
            5,
            REFINED_METALS_PRODUCTION_MODULE_ID,
            REFINED_METALS_RECIPE_ID,
        )
        .with_buys(vec![IRON_ORE_ITEM_ID])
        .with_sells(vec![REFINED_METALS_ITEM_ID])
        .with_construction_site(
            vec![
                ConstructableModuleId::ProductionModule(REFINED_METALS_PRODUCTION_MODULE_ID),
                ConstructableModuleId::ProductionModule(REFINED_METALS_PRODUCTION_MODULE_ID),
            ],
            0.0,
        );

    result
        .add(
            LocalHexPosition::from_polar(CENTER, PolarCoordinates::new(200.0, 90.0)),
            "Crystal Processor".into(),
        )
        .with_production(1, SILICA_PRODUCTION_MODULE_ID, SILICA_RECIPE_ID)
        .with_buys(vec![CRYSTAL_ORE_ITEM_ID])
        .with_sells(vec![SILICA_ITEM_ID])
        .with_construction_site(
            vec![ConstructableModuleId::ProductionModule(
                SILICA_PRODUCTION_MODULE_ID,
            )],
            0.0,
        );

    result
        .add(
            LocalHexPosition::from_polar(CENTER, PolarCoordinates::new(200.0, 300.0)),
            "Wafer Fab".into(),
        )
        .with_production(3, WAFERS_PRODUCTION_MODULE_ID, WAFERS_RECIPE_ID)
        .with_buys(vec![SILICA_ITEM_ID, HYDROGEN_ITEM_ID])
        .with_sells(vec![WAFER_ITEM_ID])
        .with_construction_site(
            vec![
                ConstructableModuleId::ProductionModule(WAFERS_PRODUCTION_MODULE_ID),
                ConstructableModuleId::ProductionModule(WAFERS_PRODUCTION_MODULE_ID),
                ConstructableModuleId::ProductionModule(WAFERS_PRODUCTION_MODULE_ID),
                ConstructableModuleId::ProductionModule(WAFERS_PRODUCTION_MODULE_ID),
            ],
            150.0,
        );

    result
        .add(LocalHexPosition::new(CENTER, Vec2::ZERO), "Shipyard".into())
        .with_shipyard(2, MOCK_SHIPYARD_MODULE_ID)
        .with_buys(vec![REFINED_METALS_ITEM_ID, WAFER_ITEM_ID]);

    result.build()
}

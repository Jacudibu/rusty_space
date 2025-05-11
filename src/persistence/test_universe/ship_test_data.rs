use crate::persistence::test_universe::coordinates::CENTER;
use common::constants;
use common::game_data::{CRYSTAL_ORE_ITEM_ID, HYDROGEN_ITEM_ID, IRON_ORE_ITEM_ID};
use common::session_data::ship_configs::{
    MOCK_CONSTRUCTION_SHIP_CONFIG_ID, MOCK_HARVESTING_SHIP_CONFIG_ID, MOCK_MINING_SHIP_CONFIG_ID,
    MOCK_TRANSPORT_SHIP_CONFIG_ID,
};
use common::types::local_hex_position::LocalHexPosition;
use hexx::Vec2;
use persistence::data::{
    AutoMineStateSaveData, SaveDataCollection, ShipBehaviorSaveData, ShipSaveData,
};
use universe_builder::builders::ship_builder::ShipBuilder;

pub fn create_test_data() -> SaveDataCollection<ShipSaveData> {
    let mut builder = ShipBuilder::default();

    let rotation_factor = (std::f32::consts::PI * 2.0) / constants::TRADE_SHIP_COUNT as f32;
    for i in 0..constants::TRADE_SHIP_COUNT {
        builder.add(
            MOCK_TRANSPORT_SHIP_CONFIG_ID,
            LocalHexPosition::new(CENTER, Vec2::ZERO),
            rotation_factor * (i as f32),
            format!("Trade Ship {i}"),
            ShipBehaviorSaveData::AutoTrade,
        );
    }

    let rotation_factor = (std::f32::consts::PI * 2.0) / constants::MINING_SHIP_COUNT as f32;
    for i in 0..constants::MINING_SHIP_COUNT {
        builder.add(
            MOCK_MINING_SHIP_CONFIG_ID,
            LocalHexPosition::new(CENTER, Vec2::ZERO),
            rotation_factor * (i as f32),
            format!("Iron Mining Ship {i}"),
            ShipBehaviorSaveData::AutoMine {
                mined_ore: IRON_ORE_ITEM_ID,
                state: AutoMineStateSaveData::Mining,
            },
        );
        builder.add(
            MOCK_MINING_SHIP_CONFIG_ID,
            LocalHexPosition::new(CENTER, Vec2::ZERO),
            rotation_factor * (i as f32),
            format!("Crystal Mining Ship {i}"),
            ShipBehaviorSaveData::AutoMine {
                mined_ore: CRYSTAL_ORE_ITEM_ID,
                state: AutoMineStateSaveData::Mining,
            },
        );
    }

    let rotation_factor = (std::f32::consts::PI * 2.0) / constants::HARVESTING_SHIP_COUNT as f32;
    for i in 0..constants::HARVESTING_SHIP_COUNT {
        builder.add(
            MOCK_HARVESTING_SHIP_CONFIG_ID,
            LocalHexPosition::new(CENTER, Vec2::ZERO),
            rotation_factor * (i as f32),
            format!("Harvesting Ship {i}"),
            ShipBehaviorSaveData::AutoHarvest {
                harvested_gas: HYDROGEN_ITEM_ID,
                state: AutoMineStateSaveData::Mining,
            },
        );
    }

    let rotation_factor = (std::f32::consts::PI * 2.0) / constants::CONSTRUCTION_SHIP_COUNT as f32;
    for i in 0..constants::CONSTRUCTION_SHIP_COUNT {
        builder.add(
            MOCK_CONSTRUCTION_SHIP_CONFIG_ID,
            LocalHexPosition::new(CENTER, Vec2::ZERO),
            rotation_factor * (i as f32),
            format!("Construction Ship {i}"),
            ShipBehaviorSaveData::AutoConstruct,
        );
    }

    builder.build()
}

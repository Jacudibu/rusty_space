use crate::constants;
use crate::game_data::{HYDROGEN_ITEM_ID, IRON_ORE_ITEM_ID};
use crate::persistence::local_hex_position::LocalHexPosition;
use crate::persistence::test_universe::coordinates::CENTER;
use crate::persistence::{SaveDataCollection, ShipBehaviorSaveData, ShipSaveData};
use crate::session_data::ship_configs::{
    MOCK_HARVESTING_SHIP_CONFIG_ID, MOCK_MINING_SHIP_CONFIG_ID, MOCK_TRANSPORT_SHIP_CONFIG_ID,
};
use crate::simulation::prelude::{Milliseconds, SimulationTimestamp};
use crate::simulation::ship_ai::AutoMineState;
use hexx::Vec2;

pub fn create_test_data() -> SaveDataCollection<ShipSaveData> {
    let mut result = SaveDataCollection::<ShipSaveData>::default();

    let rotation_factor = (std::f32::consts::PI * 2.0) / constants::TRADE_SHIP_COUNT as f32;
    for i in 0..constants::TRADE_SHIP_COUNT {
        result.add(
            MOCK_TRANSPORT_SHIP_CONFIG_ID,
            LocalHexPosition::new(CENTER, Vec2::ZERO),
            rotation_factor * (i as f32),
            format!("Trade Ship {i}"),
            ShipBehaviorSaveData::AutoTrade {
                next_idle_update: SimulationTimestamp::from(i as Milliseconds % 1000),
            },
        );
    }

    let rotation_factor = (std::f32::consts::PI * 2.0) / constants::MINING_SHIP_COUNT as f32;
    for i in 0..constants::MINING_SHIP_COUNT {
        result.add(
            MOCK_MINING_SHIP_CONFIG_ID,
            LocalHexPosition::new(CENTER, Vec2::ZERO),
            rotation_factor * (i as f32),
            format!("Mining Ship {i}"),
            ShipBehaviorSaveData::AutoMine {
                next_idle_update: SimulationTimestamp::from(i as Milliseconds % 1000),
                mined_ore: IRON_ORE_ITEM_ID,
                state: AutoMineState::Mining,
            },
        );
    }

    let rotation_factor = (std::f32::consts::PI * 2.0) / constants::HARVESTING_SHIP_COUNT as f32;
    for i in 0..constants::HARVESTING_SHIP_COUNT {
        result.add(
            MOCK_HARVESTING_SHIP_CONFIG_ID,
            LocalHexPosition::new(CENTER, Vec2::ZERO),
            rotation_factor * (i as f32),
            format!("Harvesting Ship {i}"),
            ShipBehaviorSaveData::AutoHarvest {
                next_idle_update: SimulationTimestamp::from(i as Milliseconds % 1000),
                harvested_gas: HYDROGEN_ITEM_ID,
                state: AutoMineState::Mining,
            },
        );
    }

    result
}

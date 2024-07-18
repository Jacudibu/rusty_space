use crate::constants;
use crate::persistence::local_hex_position::LocalHexPosition;
use crate::persistence::test_data::coordinates::CENTER;
use crate::persistence::{SaveDataCollection, ShipBehaviorSaveData, ShipSaveData};
use crate::ship_ai::AutoMineState;
use crate::utils::{Milliseconds, SimulationTimestamp};
use hexx::Vec2;

pub fn create_test_data() -> SaveDataCollection<ShipSaveData> {
    let mut result = SaveDataCollection::<ShipSaveData>::default();

    let rotation_factor = (std::f32::consts::PI * 2.0) / constants::TRADE_SHIP_COUNT as f32;
    for i in 0..constants::TRADE_SHIP_COUNT {
        result.add(
            LocalHexPosition::new(CENTER, Vec2::ZERO),
            rotation_factor * (i as f32),
            format!("Trade Ship {i}"),
            ShipBehaviorSaveData::AutoTrade {
                next_idle_update: SimulationTimestamp::from(i as Milliseconds),
            },
        );
    }

    let rotation_factor = (std::f32::consts::PI * 2.0) / constants::MINING_SHIP_COUNT as f32;
    for i in 0..constants::MINING_SHIP_COUNT {
        result.add(
            LocalHexPosition::new(CENTER, Vec2::ZERO),
            rotation_factor * (i as f32),
            format!("Mining Ship {i}"),
            ShipBehaviorSaveData::AutoMine {
                next_idle_update: SimulationTimestamp::from(i as Milliseconds),
                state: AutoMineState::Mining,
            },
        );
    }

    result
}

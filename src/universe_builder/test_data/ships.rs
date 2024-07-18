use crate::constants;
use crate::ship_ai::{AutoMineState, BehaviorBuilder};
use crate::universe_builder::ship_builder::ShipSpawnData;
use crate::universe_builder::test_data::coordinates::CENTER;
use crate::universe_builder::LocalHexPosition;
use crate::utils::{Milliseconds, SimulationTimestamp};
use hexx::Vec2;

pub fn create_test_data() -> ShipSpawnData {
    let mut result = ShipSpawnData::default();

    let rotation_factor = (std::f32::consts::PI * 2.0) / constants::TRADE_SHIP_COUNT as f32;
    for i in 0..constants::TRADE_SHIP_COUNT {
        result.add(
            LocalHexPosition::new(CENTER, Vec2::ZERO),
            rotation_factor * (i as f32),
            format!("Trade Ship {i}"),
            BehaviorBuilder::AutoTrade {
                next_idle_update: SimulationTimestamp::MIN,
            },
        );
    }

    let rotation_factor = (std::f32::consts::PI * 2.0) / constants::MINING_SHIP_COUNT as f32;
    for i in 0..constants::MINING_SHIP_COUNT {
        result.add(
            LocalHexPosition::new(CENTER, Vec2::ZERO),
            rotation_factor * (i as f32),
            format!("Mining Ship {i}"),
            BehaviorBuilder::AutoMine {
                next_idle_update: SimulationTimestamp::from(i as Milliseconds),
                state: AutoMineState::Mining,
            },
        );
    }

    result
}

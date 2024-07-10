use crate::constants;
use crate::ship_ai::BehaviorBuilder;
use crate::universe_builder::ship_builder::ShipSpawnData;
use crate::universe_builder::test_data::coordinates::CENTER;
use crate::universe_builder::LocalHexPosition;
use hexx::Vec2;

pub fn create_test_data() -> ShipSpawnData {
    let mut result = ShipSpawnData::default();

    let rotation_factor = (std::f32::consts::PI * 2.0) / constants::TRADE_SHIP_COUNT as f32;
    for i in 0..constants::TRADE_SHIP_COUNT {
        result.add(
            LocalHexPosition::new(CENTER, Vec2::ZERO),
            rotation_factor * (i as f32),
            format!("Trade Ship {i}"),
            BehaviorBuilder::AutoTrade,
        );
    }

    let rotation_factor = (std::f32::consts::PI * 2.0) / constants::MINING_SHIP_COUNT as f32;
    for i in 0..constants::MINING_SHIP_COUNT {
        result.add(
            LocalHexPosition::new(CENTER, Vec2::ZERO),
            rotation_factor * (i as f32),
            format!("Mining Ship {i}"),
            BehaviorBuilder::AutoMine,
        );
    }

    result
}

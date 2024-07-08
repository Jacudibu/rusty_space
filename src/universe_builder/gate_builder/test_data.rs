use crate::universe_builder::gate_builder::instance_builder::HexPosition;
use crate::universe_builder::gate_builder::resources::GateSpawnData;
use bevy::prelude::Vec2;
use hexx::Hex;

pub fn create_test_gate_data() -> GateSpawnData {
    let center = Hex::ZERO;
    let right = Hex::new(1, 0);
    let top_right = Hex::new(0, 1);
    let bottom_left = Hex::new(0, -1);

    let mut result = GateSpawnData::new();
    result.add(
        HexPosition::new(center, Vec2::new(250.0, 0.0)),
        HexPosition::new(right, Vec2::new(-250.0, 0.0)),
    );
    result.add(
        HexPosition::new(right, Vec2::new(-200.0, 130.0)),
        HexPosition::new(top_right, Vec2::new(200.0, -130.0)),
    );
    result.add(
        HexPosition::new(center, Vec2::new(-150.0, -150.0)),
        HexPosition::new(bottom_left, Vec2::new(200.0, 130.0)),
    );

    result
}

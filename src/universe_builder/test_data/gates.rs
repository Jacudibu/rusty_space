use crate::universe_builder::gate_builder::GateSpawnData;
use crate::universe_builder::local_hex_position::LocalHexPosition;
use crate::universe_builder::test_data::coordinates::{BOTTOM_LEFT, CENTER, RIGHT, TOP_RIGHT};
use bevy::prelude::Vec2;

pub fn create_test_data() -> GateSpawnData {
    let mut result = GateSpawnData::default();
    result.add(
        LocalHexPosition::new(CENTER, Vec2::new(250.0, 0.0)),
        LocalHexPosition::new(RIGHT, Vec2::new(-250.0, 0.0)),
    );
    result.add(
        LocalHexPosition::new(RIGHT, Vec2::new(-200.0, 230.0)),
        LocalHexPosition::new(TOP_RIGHT, Vec2::new(200.0, -160.0)),
    );
    result.add(
        LocalHexPosition::new(CENTER, Vec2::new(-150.0, -150.0)),
        LocalHexPosition::new(BOTTOM_LEFT, Vec2::new(200.0, 130.0)),
    );

    result
}

use crate::persistence::test_universe::coordinates::{
    BOTTOM_LEFT, CENTER, RIGHT, TOP_RIGHT, TOP_RIGHT_TOP_RIGHT,
};
use bevy::prelude::Vec2;
use common::types::local_hex_position::LocalHexPosition;
use common::types::polar_coordinates::PolarCoordinates;
use persistence::data::{GatePairSaveData, SaveDataCollection};
use universe_builder::builders::gate_builder::GatePairBuilder;

pub fn create_test_data() -> SaveDataCollection<GatePairSaveData> {
    let mut result = GatePairBuilder::default();
    result.add(
        LocalHexPosition::new(CENTER, Vec2::new(250.0, 0.0)),
        LocalHexPosition::from_polar(RIGHT, PolarCoordinates::new(240.0, 180.0)),
    );
    result.add(
        LocalHexPosition::from_polar(RIGHT, PolarCoordinates::new(360.0, 90.0)),
        LocalHexPosition::new(TOP_RIGHT, Vec2::new(200.0, -160.0)),
    );
    result.add(
        LocalHexPosition::new(TOP_RIGHT, Vec2::new(200.0, 150.0)),
        LocalHexPosition::new(TOP_RIGHT_TOP_RIGHT, Vec2::new(-200.0, -160.0)),
    );
    result.add(
        LocalHexPosition::new(CENTER, Vec2::new(-150.0, -150.0)),
        LocalHexPosition::new(BOTTOM_LEFT, Vec2::new(200.0, 130.0)),
    );

    result.build()
}

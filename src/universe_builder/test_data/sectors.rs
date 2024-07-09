use crate::components::SectorAsteroidData;
use crate::universe_builder::sector_builder::SectorSpawnData;
use crate::universe_builder::test_data::coordinates::{BOTTOM_LEFT, CENTER, RIGHT, TOP_RIGHT};
use bevy::math::Vec2;

pub fn create_test_data() -> SectorSpawnData {
    let asteroids = SectorAsteroidData {
        forward_velocity: Vec2::splat(2.0),
    };

    let mut sectors = SectorSpawnData::default();
    sectors.add(CENTER);
    sectors.add(RIGHT);
    sectors.add(TOP_RIGHT).with_asteroids(asteroids);
    sectors.add(BOTTOM_LEFT);

    sectors
}

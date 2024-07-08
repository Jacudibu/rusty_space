use crate::components::SectorAsteroidData;
use crate::universe_builder::sector_builder::resources::SectorSpawnData;
use bevy::math::Vec2;
use hexx::Hex;

pub fn create_test_sector_data() -> SectorSpawnData {
    let center = Hex::ZERO;
    let right = Hex::new(1, 0);
    let top_right = Hex::new(0, 1);
    let bottom_left = Hex::new(0, -1);

    let asteroids = SectorAsteroidData {
        forward_velocity: Vec2::splat(2.0),
    };

    let mut sectors = SectorSpawnData::new();
    sectors.add(center);
    sectors.add(right);
    sectors.add(top_right).with_asteroids(asteroids);
    sectors.add(bottom_left);

    sectors
}

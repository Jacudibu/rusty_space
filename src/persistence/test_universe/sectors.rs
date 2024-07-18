use crate::persistence::test_universe::coordinates::{BOTTOM_LEFT, CENTER, RIGHT, TOP_RIGHT};
use crate::persistence::{SaveDataCollection, SectorAsteroidSaveData, SectorSaveData};
use bevy::math::Vec2;

pub fn create_test_data() -> SaveDataCollection<SectorSaveData> {
    let asteroid_data = SectorAsteroidSaveData {
        average_velocity: Vec2::splat(2.0),
    };

    let mut sectors = SaveDataCollection::<SectorSaveData>::default();
    sectors.add(CENTER);
    sectors.add(RIGHT);
    sectors.add(TOP_RIGHT).with_asteroid_data(asteroid_data);
    sectors.add(BOTTOM_LEFT);

    sectors
}

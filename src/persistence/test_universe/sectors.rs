use crate::persistence::test_universe::coordinates::{BOTTOM_LEFT, CENTER, RIGHT, TOP_RIGHT};
use crate::persistence::{
    SaveDataCollection, SectorAsteroidSaveData, SectorSaveData, SectorStarSaveData,
};
use crate::utils::SolarMass;
use bevy::math::Vec2;

pub fn create_test_data() -> SaveDataCollection<SectorSaveData> {
    let mut sectors = SaveDataCollection::<SectorSaveData>::default();
    sectors.add(CENTER);
    sectors.add(RIGHT).with_star(SectorStarSaveData {
        mass: SolarMass::from_solar_mass(1, 0),
    });
    sectors
        .add(TOP_RIGHT)
        .with_asteroids(SectorAsteroidSaveData {
            average_velocity: Vec2::splat(2.0),
            respawning_asteroids: Vec::new(),
            live_asteroids: Vec::new(),
        });
    sectors.add(BOTTOM_LEFT);

    sectors
}

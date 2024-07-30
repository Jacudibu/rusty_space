use crate::constants;
use crate::map_layout::MapLayout;
use crate::persistence::test_universe::coordinates;
use crate::persistence::{
    ConstantOrbitSaveData, SaveDataCollection, SectorAsteroidSaveData, SectorPlanetSaveData,
    SectorSaveData, SectorStarSaveData,
};
use crate::utils::{EarthMass, SolarMass};
use bevy::math::Vec2;

pub fn create_test_data() -> SaveDataCollection<SectorSaveData> {
    let map_layout = MapLayout::default();
    let mut sectors = SaveDataCollection::<SectorSaveData>::default();
    sectors.add(coordinates::CENTER);
    sectors
        .add(coordinates::RIGHT)
        .with_star(SectorStarSaveData {
            mass: SolarMass::from_solar_mass(1, 0),
        })
        .with_planet(SectorPlanetSaveData::new(
            "Planet Alpha".to_string(),
            EarthMass::from_earth_mass(1, 0),
            ConstantOrbitSaveData {
                radius: 50.0,
                current_rotational_fraction: 0.3,
            },
        ))
        .with_planet(SectorPlanetSaveData::new(
            "Planet Beta".to_string(),
            EarthMass::from_earth_mass(1, 50),
            ConstantOrbitSaveData {
                radius: 200.0,
                current_rotational_fraction: 0.7,
            },
        ))
        .with_planet(SectorPlanetSaveData::new(
            "Planet Gamma".to_string(),
            EarthMass::from_earth_mass(2, 0),
            ConstantOrbitSaveData {
                radius: 400.0,
                current_rotational_fraction: 0.0,
            },
        ));

    sectors.add(coordinates::TOP_RIGHT).with_asteroids(
        SectorAsteroidSaveData::new()
            .with_average_velocity(Vec2::splat(1.5))
            .add_random_live_asteroids(
                coordinates::TOP_RIGHT,
                constants::ASTEROID_COUNT,
                &map_layout,
            ),
    );
    sectors
        .add(coordinates::TOP_RIGHT_TOP_RIGHT)
        .with_asteroids(
            SectorAsteroidSaveData::new()
                .with_average_velocity(Vec2::new(-0.5, -1.3))
                .add_random_live_asteroids(
                    coordinates::TOP_RIGHT_TOP_RIGHT,
                    constants::ASTEROID_COUNT,
                    &map_layout,
                ),
        );
    sectors.add(coordinates::BOTTOM_LEFT);

    sectors
}

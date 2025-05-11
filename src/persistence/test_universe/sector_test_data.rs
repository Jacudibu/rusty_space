use crate::persistence::test_universe::coordinates;
use bevy::math::Vec2;
use common::constants;
use common::game_data::{
    AsteroidManifest, CRYSTAL_ASTEROID_ID, HYDROGEN_ITEM_ID, IRON_ASTEROID_ID,
};
use common::types::celestial_mass::{CelestialMass, SolarMass};
use common::types::map_layout::MapLayout;
use common::types::polar_coordinates::PolarCoordinates;
use common::types::universe_seed::UniverseSeed;
use persistence::data::{CelestialKindSaveData, SaveDataCollection, SectorSaveData};
use universe_builder::builders::celestial_builder::SectorCelestialBuilder;
use universe_builder::builders::sector_builder::{SectorAsteroidBuilder, UniverseSectorBuilder};

const UNIVERSE_SEED: UniverseSeed = UniverseSeed::from_seed(42);

pub fn create_test_data(
    asteroid_manifest: &AsteroidManifest,
) -> SaveDataCollection<SectorSaveData> {
    let map_layout = MapLayout::default();
    let mut sectors = UniverseSectorBuilder::default();
    sectors.add(coordinates::CENTER);

    sectors
        .add(coordinates::RIGHT)
        .with_celestial(
            SectorCelestialBuilder::new(
                CelestialKindSaveData::Star,
                PolarCoordinates {
                    radial_distance: 0.0,
                    angle: 0.0,
                }
                .to_cartesian(),
            )
            .with_mass(CelestialMass::SolarMass(SolarMass::from_solar_mass(1, 0))),
        )
        .with_celestial(SectorCelestialBuilder::new(
            CelestialKindSaveData::Terrestrial,
            PolarCoordinates {
                radial_distance: 120.0,
                angle: 100.0,
            }
            .to_cartesian(),
        ))
        .with_celestial(SectorCelestialBuilder::new(
            CelestialKindSaveData::Terrestrial,
            PolarCoordinates {
                radial_distance: 240.0,
                angle: 210.0,
            }
            .to_cartesian(),
        ))
        .with_celestial(SectorCelestialBuilder::new(
            CelestialKindSaveData::GasGiant {
                resources: vec![HYDROGEN_ITEM_ID],
            },
            PolarCoordinates {
                radial_distance: 360.0,
                angle: 0.0,
            }
            .to_cartesian(),
        ));

    sectors.add(coordinates::TOP_RIGHT).with_asteroids(
        SectorAsteroidBuilder::new()
            .with_average_velocity(Vec2::splat(1.5))
            .add_random_live_asteroids(
                coordinates::TOP_RIGHT,
                constants::ASTEROID_COUNT,
                &UNIVERSE_SEED,
                &map_layout,
                asteroid_manifest,
                IRON_ASTEROID_ID,
            ),
    );
    sectors
        .add(coordinates::TOP_RIGHT_TOP_RIGHT)
        .with_asteroids(
            SectorAsteroidBuilder::new()
                .with_average_velocity(Vec2::new(-0.5, -1.3))
                .add_random_live_asteroids(
                    coordinates::TOP_RIGHT_TOP_RIGHT,
                    constants::ASTEROID_COUNT,
                    &UNIVERSE_SEED,
                    &map_layout,
                    asteroid_manifest,
                    CRYSTAL_ASTEROID_ID,
                ),
        );
    sectors.add(coordinates::BOTTOM_LEFT);
    sectors.build()
}

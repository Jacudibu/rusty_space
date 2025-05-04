use crate::game_data::{AsteroidManifest, CRYSTAL_ASTEROID_ID, HYDROGEN_ITEM_ID, IRON_ASTEROID_ID};
use crate::map_layout::MapLayout;
use crate::persistence::test_universe::coordinates;
use crate::persistence::{
    PlanetKindSaveData, SaveDataCollection, SectorAsteroidSaveData, SectorPlanetSaveData,
    SectorSaveData, SectorStarSaveData,
};
use crate::utils::UniverseSeed;
use crate::utils::polar_coordinates::PolarCoordinates;
use bevy::math::Vec2;
use common::constants;

const UNIVERSE_SEED: UniverseSeed = UniverseSeed::from_seed(42);

pub fn create_test_data(
    asteroid_manifest: &AsteroidManifest,
) -> SaveDataCollection<SectorSaveData> {
    let map_layout = MapLayout::default();
    let mut sectors = SaveDataCollection::<SectorSaveData>::default();
    sectors.add(coordinates::CENTER);

    sectors
        .add(coordinates::RIGHT)
        .with_star(SectorStarSaveData::new())
        .with_planet(SectorPlanetSaveData::new(
            PolarCoordinates {
                radial_distance: 120.0,
                angle: 100.0,
            }
            .to_cartesian(),
        ))
        .with_planet(SectorPlanetSaveData::new(
            PolarCoordinates {
                radial_distance: 240.0,
                angle: 210.0,
            }
            .to_cartesian(),
        ))
        .with_planet(
            SectorPlanetSaveData::new(
                PolarCoordinates {
                    radial_distance: 360.0,
                    angle: 0.0,
                }
                .to_cartesian(),
            )
            .with_kind(PlanetKindSaveData::GasGiant {
                resources: vec![HYDROGEN_ITEM_ID],
            }),
        );

    sectors.add(coordinates::TOP_RIGHT).with_asteroids(
        SectorAsteroidSaveData::new()
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
            SectorAsteroidSaveData::new()
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

    sectors
}

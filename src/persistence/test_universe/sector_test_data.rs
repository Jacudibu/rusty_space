use crate::constants;
use crate::game_data::{MOCK_ASTEROID_ID, MOCK_ITEM_ORE_ID};
use crate::map_layout::MapLayout;
use crate::persistence::test_universe::coordinates;
use crate::persistence::{
    ConstantOrbitSaveData, PlanetKindSaveData, SaveDataCollection, SectorAsteroidSaveData,
    SectorPlanetSaveData, SectorSaveData, SectorStarSaveData,
};
use crate::utils::UniverseSeed;
use bevy::math::Vec2;

const UNIVERSE_SEED: UniverseSeed = UniverseSeed::from_seed(42);

pub fn create_test_data() -> SaveDataCollection<SectorSaveData> {
    let map_layout = MapLayout::default();
    let mut sectors = SaveDataCollection::<SectorSaveData>::default();
    sectors.add(coordinates::CENTER);

    sectors
        .add(coordinates::RIGHT)
        .with_star(SectorStarSaveData::new())
        .with_planet(SectorPlanetSaveData::new(
            ConstantOrbitSaveData::new(50.0).with_current_rotational_fraction(0.3),
        ))
        .with_planet(SectorPlanetSaveData::new(
            ConstantOrbitSaveData::new(200.0).with_current_rotational_fraction(0.7),
        ))
        .with_planet(
            SectorPlanetSaveData::new(ConstantOrbitSaveData::new(400.0))
                .with_kind(PlanetKindSaveData::GasGiant),
        );

    sectors.add(coordinates::TOP_RIGHT).with_asteroids(
        SectorAsteroidSaveData::new()
            // TODO: This should be rewritten to instead be fed AsteroidData, random_live_asteroids spawning can then just use that data.
            .with_asteroid_types(vec![MOCK_ITEM_ORE_ID])
            .with_average_velocity(Vec2::splat(1.5))
            .add_random_live_asteroids(
                coordinates::TOP_RIGHT,
                MOCK_ITEM_ORE_ID,
                constants::ASTEROID_COUNT,
                &UNIVERSE_SEED,
                &map_layout,
            ),
    );
    sectors
        .add(coordinates::TOP_RIGHT_TOP_RIGHT)
        .with_asteroids(
            SectorAsteroidSaveData::new()
                // TODO: This should be rewritten to instead be fed AsteroidData
                .with_asteroid_types(vec![MOCK_ITEM_ORE_ID])
                .with_average_velocity(Vec2::new(-0.5, -1.3))
                .add_random_live_asteroids(
                    coordinates::TOP_RIGHT_TOP_RIGHT,
                    MOCK_ITEM_ORE_ID,
                    constants::ASTEROID_COUNT,
                    &UNIVERSE_SEED,
                    &map_layout,
                ),
        );
    sectors.add(coordinates::BOTTOM_LEFT);

    sectors
}

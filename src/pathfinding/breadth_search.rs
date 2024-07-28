use crate::components::{Sector, SectorAsteroidComponent};
use crate::persistence::SectorIdMap;
use crate::utils::SectorEntity;
use bevy::prelude::Query;

// TODO: Find a way to replace this with a generic search query + function
pub fn breadth_search(
    all_sectors: &Query<&Sector>,
    sector_id_map: &SectorIdMap,
    from: SectorEntity,
    max_range: u32,
    sector_search_query: &Query<&SectorAsteroidComponent>,
) -> Option<SectorEntity>
where
{
    None
}

#[cfg(test)]
mod test {
    use crate::components::{Sector, SectorAsteroidComponent};
    use crate::pathfinding::breadth_search::breadth_search;
    use crate::persistence::local_hex_position::LocalHexPosition;
    use crate::persistence::{SectorAsteroidSaveData, SectorIdMap, UniverseSaveData};
    use bevy::ecs::system::RunSystemOnce;
    use bevy::prelude::{Query, Res, Vec2, World};
    use hexx::Hex;

    const LEFT: Hex = Hex::new(-1, 0);
    const CENTER: Hex = Hex::new(0, 0);
    const RIGHT: Hex = Hex::new(1, 0);

    fn test_breadth_search(
        world: &mut World,
        from_sector: Hex,
        max_range: u32,
        expected_result: Option<Hex>,
    ) {
        world.run_system_once(
            move |sectors: Query<&Sector>,
                  sector_id_map: Res<SectorIdMap>,
                  search_query: Query<&SectorAsteroidComponent>| {
                let from_entity = sector_id_map.id_to_entity()[&from_sector];

                let result = breadth_search(
                    &sectors,
                    &sector_id_map,
                    from_entity,
                    max_range,
                    &search_query,
                );
                if let Some(expected_result) = expected_result {
                    assert_eq!(
                        expected_result,
                        sector_id_map.entity_to_id()[&result.unwrap()]
                    );
                } else {
                    assert!(result.is_none());
                }
            },
        );
    }

    #[test]
    fn find_neighbor_with_asteroids() {
        let mut universe = UniverseSaveData::default();
        universe.sectors.add(LEFT);
        universe.sectors.add(CENTER);
        universe
            .sectors
            .add(RIGHT)
            .with_asteroids(SectorAsteroidSaveData {
                // TODO: That probably also needs a builder
                average_velocity: Vec2::ONE,
                live_asteroids: Vec::new(),
                respawning_asteroids: Vec::new(),
            });
        universe.gate_pairs.add(
            LocalHexPosition::new(LEFT, Vec2::X),
            LocalHexPosition::new(CENTER, Vec2::NEG_X),
        );
        universe.gate_pairs.add(
            LocalHexPosition::new(CENTER, Vec2::X),
            LocalHexPosition::new(RIGHT, Vec2::NEG_X),
        );

        let mut app = universe.build_test_app();
        let world = app.world_mut();

        test_breadth_search(world, CENTER, 5, Some(RIGHT));
    }

    #[test]
    fn find_neighbor_with_asteroids_does_not_find_anything_outside_of_max_range() {
        let mut universe = UniverseSaveData::default();
        universe.sectors.add(LEFT);
        universe.sectors.add(CENTER);
        universe
            .sectors
            .add(RIGHT)
            .with_asteroids(SectorAsteroidSaveData {
                // TODO: That probably also needs a builder
                average_velocity: Vec2::ONE,
                live_asteroids: Vec::new(),
                respawning_asteroids: Vec::new(),
            });
        universe.gate_pairs.add(
            LocalHexPosition::new(LEFT, Vec2::X),
            LocalHexPosition::new(CENTER, Vec2::NEG_X),
        );
        universe.gate_pairs.add(
            LocalHexPosition::new(CENTER, Vec2::X),
            LocalHexPosition::new(RIGHT, Vec2::NEG_X),
        );

        let mut app = universe.build_test_app();
        let world = app.world_mut();

        test_breadth_search(world, LEFT, 1, None);
    }
}

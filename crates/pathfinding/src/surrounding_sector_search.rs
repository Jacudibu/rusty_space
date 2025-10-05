use bevy::ecs::query::{QueryData, ReadOnlyQueryData};
use bevy::platform::collections::HashSet;
use bevy::platform::hash::FixedHasher;
use bevy::prelude::Query;
use common::components::Sector;
use common::types::entity_wrappers::SectorEntity;
use std::cmp::Ordering;
use std::ops::Not;

#[derive(PartialEq, Eq)]
pub struct SearchResult {
    pub distance: u8,
    pub sector: SectorEntity,
}

impl Ord for SearchResult {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance
            .cmp(&other.distance)
            .then_with(|| self.sector.cmp(&other.sector))
    }
}

impl PartialOrd for SearchResult {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Performs a breadth-first search on the sectors surrounding `from`, reaching up to (and including) `max_range` sectors away.
///
/// # Returns
/// A Vec containing the [`SearchResult`]s matching the provided `search_fn`, ordered by their sector distance to `from`.
pub fn surrounding_sector_search<'a, TSearchQueryData, TSearchFunction>(
    all_sectors: &Query<&Sector>,
    from: SectorEntity,
    min_range: u8,
    max_range: u8,
    sector_search_query: &'a Query<'a, '_, TSearchQueryData>,
    search_fn: TSearchFunction,
) -> Vec<SearchResult>
where
    TSearchQueryData: QueryData + ReadOnlyQueryData,
    TSearchFunction: Fn(TSearchQueryData::Item<'a, '_>) -> bool,
{
    let mut visited = HashSet::<&SectorEntity, FixedHasher>::default();
    let mut next = vec![&from];
    let mut result = Vec::new();

    let mut current_depth = 0;
    while current_depth <= max_range && next.is_empty().not() {
        let mut next_next = Vec::new();
        for sector_entity in next.into_iter() {
            visited.insert(sector_entity);

            let sector = all_sectors.get(sector_entity.into()).unwrap();

            if current_depth >= min_range {
                if let Ok(search_components) = sector_search_query.get(sector_entity.into()) {
                    // TODO: TSearchFunction could return an Option<T>, which could be persisted in SearchResult<T>
                    if search_fn(search_components) {
                        result.push(SearchResult {
                            distance: current_depth,
                            sector: *sector_entity,
                        });
                    }
                }
            }

            for (to_sector, _) in &sector.gates {
                if !visited.contains(to_sector) {
                    next_next.push(to_sector);
                }
            }
        }

        next = next_next;
        current_depth += 1;
    }

    result
}

#[cfg(test)]
mod test {
    use crate::surrounding_sector_search::surrounding_sector_search;
    use bevy::ecs::system::RunSystemOnce;
    use bevy::prelude::{Query, Res, Vec2, World};
    use common::components::{Sector, SectorWithAsteroids};
    use common::types::entity_id_map::SectorIdMap;
    use common::types::local_hex_position::LocalHexPosition;
    use hexx::Hex;
    use test_utils::test_app::TestApp;
    use universe_builder::sector_builder::SectorAsteroidBuilder;

    const LEFT: Hex = Hex::new(-1, 0);
    const CENTER: Hex = Hex::new(0, 0);
    const RIGHT: Hex = Hex::new(1, 0);

    fn has_asteroids(_: &SectorWithAsteroids) -> bool {
        true
    }

    fn test_breadth_search(
        world: &mut World,
        from_sector: Hex,
        min_range: u8,
        max_range: u8,
        expected_result: Vec<(u8, Hex)>,
    ) {
        world
            .run_system_once(
                move |sectors: Query<&Sector>,
                      sector_id_map: Res<SectorIdMap>,
                      search_query: Query<&SectorWithAsteroids>| {
                    let from_entity = sector_id_map.id_to_entity()[&from_sector];

                    let result = surrounding_sector_search(
                        &sectors,
                        from_entity,
                        min_range,
                        max_range,
                        &search_query,
                        has_asteroids,
                    );

                    let transformed_result: Vec<(u8, Hex)> = result
                        .iter()
                        .map(|x| (x.distance, sector_id_map.entity_to_id()[&x.sector]))
                        .collect();

                    assert_eq!(expected_result, transformed_result);
                },
            )
            .unwrap();
    }

    #[test]
    fn single_result_direct_neighbor() {
        let mut test_app = TestApp::default();
        test_app.sectors.add(LEFT);
        test_app.sectors.add(CENTER);
        test_app
            .sectors
            .add(RIGHT)
            .with_asteroids(SectorAsteroidBuilder::new());
        test_app.gate_pairs.add(
            LocalHexPosition::new(LEFT, Vec2::X),
            LocalHexPosition::new(CENTER, Vec2::NEG_X),
        );
        test_app.gate_pairs.add(
            LocalHexPosition::new(CENTER, Vec2::X),
            LocalHexPosition::new(RIGHT, Vec2::NEG_X),
        );

        let mut app = test_app.build();
        let world = app.world_mut();

        test_breadth_search(world, CENTER, 0, 5, vec![(1, RIGHT)]);
    }

    #[test]
    fn multiple_results() {
        let mut test_app = TestApp::default();
        test_app.sectors.add(LEFT);
        test_app
            .sectors
            .add(CENTER)
            .with_asteroids(SectorAsteroidBuilder::new());
        test_app
            .sectors
            .add(RIGHT)
            .with_asteroids(SectorAsteroidBuilder::new());
        test_app.gate_pairs.add(
            LocalHexPosition::new(LEFT, Vec2::X),
            LocalHexPosition::new(CENTER, Vec2::NEG_X),
        );
        test_app.gate_pairs.add(
            LocalHexPosition::new(CENTER, Vec2::X),
            LocalHexPosition::new(RIGHT, Vec2::NEG_X),
        );

        let mut app = test_app.build();
        let world = app.world_mut();

        test_breadth_search(world, LEFT, 0, 2, vec![(1, CENTER), (2, RIGHT)]);
    }

    #[test]
    fn min_range() {
        let mut test_app = TestApp::default();
        test_app
            .sectors
            .add(LEFT)
            .with_asteroids(SectorAsteroidBuilder::new());
        test_app
            .sectors
            .add(CENTER)
            .with_asteroids(SectorAsteroidBuilder::new());
        test_app
            .sectors
            .add(RIGHT)
            .with_asteroids(SectorAsteroidBuilder::new());
        test_app.gate_pairs.add(
            LocalHexPosition::new(LEFT, Vec2::X),
            LocalHexPosition::new(CENTER, Vec2::NEG_X),
        );
        test_app.gate_pairs.add(
            LocalHexPosition::new(CENTER, Vec2::X),
            LocalHexPosition::new(RIGHT, Vec2::NEG_X),
        );

        let mut app = test_app.build();
        let world = app.world_mut();

        test_breadth_search(world, LEFT, 2, 2, vec![(2, RIGHT)]);
    }

    #[test]
    fn max_range() {
        let mut test_app = TestApp::default();
        test_app.sectors.add(LEFT);
        test_app.sectors.add(CENTER);
        test_app
            .sectors
            .add(RIGHT)
            .with_asteroids(SectorAsteroidBuilder::new());
        test_app.gate_pairs.add(
            LocalHexPosition::new(LEFT, Vec2::X),
            LocalHexPosition::new(CENTER, Vec2::NEG_X),
        );
        test_app.gate_pairs.add(
            LocalHexPosition::new(CENTER, Vec2::X),
            LocalHexPosition::new(RIGHT, Vec2::NEG_X),
        );

        let mut app = test_app.build();
        let world = app.world_mut();

        test_breadth_search(world, LEFT, 0, 1, vec![]);
    }
}

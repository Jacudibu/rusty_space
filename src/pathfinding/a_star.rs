use crate::components::Sector;
use crate::pathfinding::search_node::{SearchNode, GATE_COST};
use crate::pathfinding::PathElement;
use crate::simulation::transform::simulation_transform::SimulationTransform;
use crate::utils::SectorEntity;
use bevy::prelude::{Query, Vec2};
use bevy::utils::HashMap;
use std::collections::BinaryHeap;

pub fn a_star(
    sectors: &Query<&Sector>,
    gate_positions: &Query<&SimulationTransform>,
    from: SectorEntity,
    from_position: Vec2,
    to: SectorEntity,
    to_position: Option<Vec2>,
) -> Option<Vec<PathElement>> {
    let mut open = BinaryHeap::new();
    let mut costs: HashMap<PathElement, u32> = HashMap::new();

    for (sector, gate_pair) in &sectors.get(from.into()).unwrap().gates {
        let cost = cost(
            sectors,
            gate_positions,
            from,
            from_position,
            *sector,
            to,
            to_position,
        )
        .unwrap();

        let this = PathElement::new(*sector, *gate_pair);

        #[cfg(test)]
        {
            let from = sectors.get(from.into()).unwrap();
            let to = sectors.get(sector.into()).unwrap();
            println!(
                "init with [{},{}] -> [{},{}] == {cost}",
                from.coordinate.x, from.coordinate.y, to.coordinate.x, to.coordinate.y,
            );
        }

        costs.insert(this, cost);
        open.push(SearchNode {
            sector: *sector,
            gate_pair: *gate_pair,
            cost,
        });
    }

    // <Next, Previous> - used to reconstruct the optimal path later
    let mut came_from: HashMap<PathElement, PathElement> = HashMap::new();

    while let Some(node) = open.pop() {
        #[cfg(test)]
        {
            let from = sectors.get(node.sector.into()).unwrap();
            println!("Now in [{},{}]", from.coordinate.x, from.coordinate.y)
        }

        if node.sector == to {
            #[cfg(test)]
            {
                println!("In target sector. Total Cost: {}", node.cost);
            }

            return Some(reconstruct_path(&came_from, node));
        }

        let current = PathElement::new(node.sector, node.gate_pair);
        let current_cost = costs[&current];
        for (next_sector, gate_pair) in &sectors.get(node.sector.into()).unwrap().gates {
            let gate_pos = gate_positions
                .get(node.gate_pair.to.into())
                .unwrap()
                .translation;

            let Some(cost) = cost(
                sectors,
                gate_positions,
                node.sector,
                gate_pos,
                *next_sector,
                to,
                to_position,
            ) else {
                // Technically this never happens... yet. Maybe once we have initial sector fog of war, though.
                continue;
            };

            let neighbor = PathElement::new(*next_sector, *gate_pair);
            let neighbor_cost = current_cost + cost;

            if !costs.contains_key(&neighbor) || costs[&neighbor] > neighbor_cost {
                came_from.insert(neighbor, current);
                costs.insert(neighbor, neighbor_cost);

                #[cfg(test)]
                {
                    let from = sectors.get(node.sector.into()).unwrap();
                    let to = sectors.get(next_sector.into()).unwrap();
                    println!(
                        "[{},{}] -> [{},{}] == {neighbor_cost}",
                        from.coordinate.x, from.coordinate.y, to.coordinate.x, to.coordinate.y,
                    );
                }

                open.push(SearchNode {
                    sector: neighbor.exit_sector,
                    gate_pair: neighbor.gate_pair,
                    cost: neighbor_cost,
                })
            }
        }
    }

    None
}

fn cost(
    sectors: &Query<&Sector>,
    gate_positions: &Query<&SimulationTransform>,
    from_sector: SectorEntity,
    from_pos_in_sector: Vec2,
    to_sector: SectorEntity,
    full_path_target_sector: SectorEntity,
    full_path_target_pos: Option<Vec2>,
) -> Option<u32> {
    if from_sector == to_sector {
        return if let Some(full_path_target_pos) = full_path_target_pos {
            Some(
                from_pos_in_sector
                    .distance_squared(full_path_target_pos)
                    .abs() as u32,
            )
        } else {
            Some(0)
        };
    }

    let current_sector = sectors.get(from_sector.into()).unwrap();
    let gate_pair = current_sector.gates.get(&to_sector)?;

    let enter_gate = gate_positions.get(gate_pair.from.into()).unwrap();
    let mut result = from_pos_in_sector
        .distance_squared(enter_gate.translation)
        .abs() as u32
        + GATE_COST;

    if to_sector == full_path_target_sector {
        if let Some(target_pos) = full_path_target_pos {
            // This will make sure that we truly take the shortest route to the target position
            let next_gate_pos = gate_positions.get(gate_pair.to.into()).unwrap().translation;
            result += target_pos.distance_squared(next_gate_pos).abs() as u32;
        }
    }

    Some(result)
}

fn reconstruct_path(
    came_from: &HashMap<PathElement, PathElement>,
    end: SearchNode,
) -> Vec<PathElement> {
    let mut path: Vec<PathElement> = std::iter::successors(
        Some(PathElement {
            exit_sector: end.sector,
            gate_pair: end.gate_pair,
        }),
        move |current| came_from.get(current).copied(),
    )
    .collect();
    path.reverse();
    path
}

#[cfg(test)]
mod test {
    use crate::components::Sector;
    use crate::pathfinding::find_path;
    use crate::persistence::local_hex_position::LocalHexPosition;
    use crate::persistence::{SectorIdMap, UniverseSaveData};
    use crate::simulation::transform::simulation_transform::SimulationTransform;
    use bevy::ecs::system::RunSystemOnce;
    use bevy::prelude::{Query, Res, Vec2};
    use hexx::Hex;

    const LEFT2: Hex = Hex::new(-2, 0);
    const LEFT: Hex = Hex::new(-1, 0);
    const CENTER_LEFT_TOP: Hex = Hex::new(-1, 1);
    const CENTER: Hex = Hex::new(0, 0);
    const CENTER_RIGHT_TOP: Hex = Hex::new(1, 1);
    const RIGHT: Hex = Hex::new(1, 0);
    const RIGHT2: Hex = Hex::new(2, 0);

    #[test]
    fn find_path_to_neighbor() {
        let mut universe = UniverseSaveData::default();
        universe.sectors.add(CENTER);
        universe.sectors.add(RIGHT);
        universe.gate_pairs.add(
            LocalHexPosition::new(CENTER, Vec2::ZERO),
            LocalHexPosition::new(RIGHT, Vec2::ZERO),
        );

        let mut app = universe.build_test_app();
        let world = app.world_mut();

        world.run_system_once(
            |sectors: Query<&Sector>,
             transforms: Query<&SimulationTransform>,
             sector_id_map: Res<SectorIdMap>| {
                let result = find_path(
                    &sectors,
                    &transforms,
                    sector_id_map.id_to_entity()[&CENTER],
                    Vec2::ZERO,
                    sector_id_map.id_to_entity()[&RIGHT],
                    None,
                )
                .unwrap();

                assert_eq!(result.len(), 1);
                assert_eq!(result[0].exit_sector, sector_id_map.id_to_entity()[&RIGHT]);
            },
        );
    }

    #[test]
    fn find_path_through_single_sector() {
        let mut universe = UniverseSaveData::default();
        universe.sectors.add(LEFT);
        universe.sectors.add(CENTER);
        universe.sectors.add(RIGHT);
        universe.gate_pairs.add(
            LocalHexPosition::new(LEFT, Vec2::NEG_X),
            LocalHexPosition::new(CENTER, Vec2::NEG_X),
        );
        universe.gate_pairs.add(
            LocalHexPosition::new(CENTER, Vec2::X),
            LocalHexPosition::new(RIGHT, Vec2::X),
        );

        let mut app = universe.build_test_app();
        let world = app.world_mut();

        world.run_system_once(
            |sectors: Query<&Sector>,
             transforms: Query<&SimulationTransform>,
             sector_id_map: Res<SectorIdMap>| {
                let result = find_path(
                    &sectors,
                    &transforms,
                    sector_id_map.id_to_entity()[&LEFT],
                    Vec2::ZERO,
                    sector_id_map.id_to_entity()[&RIGHT],
                    None,
                )
                .unwrap();

                assert_eq!(result.len(), 2);
                assert_eq!(result[0].exit_sector, sector_id_map.id_to_entity()[&CENTER]);
                assert_eq!(result[1].exit_sector, sector_id_map.id_to_entity()[&RIGHT]);
            },
        );
    }

    #[test]
    fn find_path_through_multiple_sectors() {
        let mut universe = UniverseSaveData::default();
        universe.sectors.add(LEFT);
        universe.sectors.add(LEFT2);
        universe.sectors.add(CENTER);
        universe.sectors.add(RIGHT);
        universe.sectors.add(RIGHT2);
        universe.gate_pairs.add(
            LocalHexPosition::new(LEFT2, Vec2::X),
            LocalHexPosition::new(LEFT, Vec2::NEG_X),
        );
        universe.gate_pairs.add(
            LocalHexPosition::new(LEFT, Vec2::X),
            LocalHexPosition::new(CENTER, Vec2::NEG_X),
        );
        universe.gate_pairs.add(
            LocalHexPosition::new(CENTER, Vec2::X),
            LocalHexPosition::new(RIGHT, Vec2::NEG_X),
        );
        universe.gate_pairs.add(
            LocalHexPosition::new(RIGHT, Vec2::X),
            LocalHexPosition::new(RIGHT2, Vec2::NEG_X),
        );

        let mut app = universe.build_test_app();
        let world = app.world_mut();

        world.run_system_once(
            |sectors: Query<&Sector>,
             transforms: Query<&SimulationTransform>,
             sector_id_map: Res<SectorIdMap>| {
                let result = find_path(
                    &sectors,
                    &transforms,
                    sector_id_map.id_to_entity()[&LEFT2],
                    Vec2::ZERO,
                    sector_id_map.id_to_entity()[&RIGHT2],
                    None,
                )
                .unwrap();

                assert_eq!(result.len(), 4);
                assert_eq!(result[0].exit_sector, sector_id_map.id_to_entity()[&LEFT]);
                assert_eq!(result[1].exit_sector, sector_id_map.id_to_entity()[&CENTER]);
                assert_eq!(result[2].exit_sector, sector_id_map.id_to_entity()[&RIGHT]);
                assert_eq!(result[3].exit_sector, sector_id_map.id_to_entity()[&RIGHT2]);
            },
        );
    }

    #[test]
    fn find_path_through_multiple_sectors_with_multiple_routes_returns_shortest_path() {
        let mut universe = UniverseSaveData::default();
        universe.sectors.add(LEFT);
        universe.sectors.add(LEFT2);
        universe.sectors.add(CENTER_LEFT_TOP);
        universe.sectors.add(CENTER);
        universe.sectors.add(CENTER_RIGHT_TOP);
        universe.sectors.add(RIGHT);
        universe.sectors.add(RIGHT2);
        universe.gate_pairs.add(
            LocalHexPosition::new(LEFT2, Vec2::X),
            LocalHexPosition::new(LEFT, Vec2::NEG_X),
        );
        universe.gate_pairs.add(
            LocalHexPosition::new(LEFT, Vec2::X),
            LocalHexPosition::new(CENTER, Vec2::NEG_X),
        );
        universe.gate_pairs.add(
            LocalHexPosition::new(CENTER, Vec2::X),
            LocalHexPosition::new(RIGHT, Vec2::NEG_X),
        );
        universe.gate_pairs.add(
            LocalHexPosition::new(RIGHT, Vec2::X),
            LocalHexPosition::new(RIGHT2, Vec2::NEG_X),
        );

        universe.gate_pairs.add(
            LocalHexPosition::new(LEFT, Vec2::X),
            LocalHexPosition::new(CENTER_LEFT_TOP, Vec2::NEG_X),
        );
        universe.gate_pairs.add(
            LocalHexPosition::new(CENTER_LEFT_TOP, Vec2::X),
            LocalHexPosition::new(CENTER_RIGHT_TOP, Vec2::NEG_X),
        );
        universe.gate_pairs.add(
            LocalHexPosition::new(CENTER_RIGHT_TOP, Vec2::X),
            LocalHexPosition::new(RIGHT, Vec2::NEG_X),
        );

        let mut app = universe.build_test_app();
        let world = app.world_mut();

        world.run_system_once(
            |sectors: Query<&Sector>,
             transforms: Query<&SimulationTransform>,
             sector_id_map: Res<SectorIdMap>| {
                let from = sector_id_map.id_to_entity()[&LEFT2];
                let from_pos = sectors.get(from.into()).unwrap();
                let result = find_path(
                    &sectors,
                    &transforms,
                    from,
                    from_pos.world_pos,
                    sector_id_map.id_to_entity()[&RIGHT2],
                    None,
                )
                .unwrap();

                assert_eq!(result.len(), 4);
                assert_eq!(result[0].exit_sector, sector_id_map.id_to_entity()[&LEFT]);
                assert_eq!(result[1].exit_sector, sector_id_map.id_to_entity()[&CENTER]);
                assert_eq!(result[2].exit_sector, sector_id_map.id_to_entity()[&RIGHT]);
                assert_eq!(result[3].exit_sector, sector_id_map.id_to_entity()[&RIGHT2]);
            },
        );
    }

    #[test]
    fn find_path_to_position_in_direct_neighbor_but_a_more_efficient_path_through_other_sector() {
        let mut universe = UniverseSaveData::default();
        universe.sectors.add(LEFT);
        universe.sectors.add(CENTER);
        universe.sectors.add(RIGHT);
        universe.gate_pairs.add(
            LocalHexPosition::new(LEFT, Vec2::X),
            LocalHexPosition::new(CENTER, Vec2::NEG_X),
        );
        universe.gate_pairs.add(
            LocalHexPosition::new(CENTER, Vec2::X),
            LocalHexPosition::new(RIGHT, Vec2::NEG_X),
        );
        universe.gate_pairs.add(
            LocalHexPosition::new(LEFT, Vec2::ZERO), // Right next to our starting position
            LocalHexPosition::new(RIGHT, Vec2::X * -20000.0), // But SO far away afterwards~
        );

        let mut app = universe.build_test_app();
        let world = app.world_mut();

        world.run_system_once(
            move |sectors: Query<&Sector>,
                  transforms: Query<&SimulationTransform>,
                  sector_id_map: Res<SectorIdMap>| {
                let from = sector_id_map.id_to_entity()[&LEFT];
                let from_pos = sectors.get(from.into()).unwrap();
                let result = find_path(
                    &sectors,
                    &transforms,
                    from,
                    from_pos.world_pos,
                    sector_id_map.id_to_entity()[&RIGHT],
                    Some(Vec2::ZERO),
                )
                .unwrap();

                assert_eq!(result.len(), 2);
                assert_eq!(result[0].exit_sector, sector_id_map.id_to_entity()[&CENTER]);
                assert_eq!(result[1].exit_sector, sector_id_map.id_to_entity()[&RIGHT]);
            },
        );
    }

    #[test]
    fn find_path_to_position_with_multiple_gates_to_target_sector() {
        let mut universe = UniverseSaveData::default();
        universe.sectors.add(LEFT2);
        universe.sectors.add(LEFT);
        universe.sectors.add(CENTER);
        universe.sectors.add(RIGHT);
        universe.gate_pairs.add(
            LocalHexPosition::new(LEFT2, Vec2::X),
            LocalHexPosition::new(LEFT, Vec2::NEG_X),
        );
        universe.gate_pairs.add(
            LocalHexPosition::new(LEFT, Vec2::X),
            LocalHexPosition::new(CENTER, Vec2::NEG_X),
        );
        universe.gate_pairs.add(
            LocalHexPosition::new(CENTER, Vec2::X),
            LocalHexPosition::new(RIGHT, Vec2::NEG_X),
        );
        universe.gate_pairs.add(
            LocalHexPosition::new(LEFT, Vec2::ZERO), // Easier to reach
            LocalHexPosition::new(RIGHT, Vec2::X * 500.0), // But SO far away~
        );

        let mut app = universe.build_test_app();
        let world = app.world_mut();

        world.run_system_once(
            move |sectors: Query<&Sector>,
                  transforms: Query<&SimulationTransform>,
                  sector_id_map: Res<SectorIdMap>| {
                let from = sector_id_map.id_to_entity()[&LEFT2];
                let from_pos = sectors.get(from.into()).unwrap();
                let result = find_path(
                    &sectors,
                    &transforms,
                    from,
                    from_pos.world_pos,
                    sector_id_map.id_to_entity()[&RIGHT],
                    Some(Vec2::ZERO),
                )
                .unwrap();

                assert_eq!(result.len(), 3);
                assert_eq!(result[0].exit_sector, sector_id_map.id_to_entity()[&LEFT]);
                assert_eq!(result[1].exit_sector, sector_id_map.id_to_entity()[&CENTER]);
                assert_eq!(result[2].exit_sector, sector_id_map.id_to_entity()[&RIGHT]);
            },
        );
    }

    #[ignore = "Pathfinding where from_pos == to_pos just shouldn't happen right now"]
    #[test]
    fn find_path_to_position_in_self_but_path_through_other_sector_is_shorter() {
        let mut universe = UniverseSaveData::default();
        universe.sectors.add(CENTER);
        universe.sectors.add(RIGHT);

        let from_pos = Vec2::X * 1000.0;
        universe.gate_pairs.add(
            LocalHexPosition::new(CENTER, from_pos),
            LocalHexPosition::new(RIGHT, Vec2::NEG_X),
        );
        universe.gate_pairs.add(
            LocalHexPosition::new(RIGHT, Vec2::X),
            LocalHexPosition::new(CENTER, -from_pos), // Cheesy shortcut!
        );

        let mut app = universe.build_test_app();
        let world = app.world_mut();

        world.run_system_once(
            move |sectors: Query<&Sector>,
                  transforms: Query<&SimulationTransform>,
                  sector_id_map: Res<SectorIdMap>| {
                let result = find_path(
                    &sectors,
                    &transforms,
                    sector_id_map.id_to_entity()[&CENTER],
                    from_pos,
                    sector_id_map.id_to_entity()[&CENTER],
                    Some(-from_pos),
                )
                .unwrap();

                assert_eq!(result.len(), 2);
                assert_eq!(result[0].exit_sector, sector_id_map.id_to_entity()[&RIGHT]);
                assert_eq!(result[1].exit_sector, sector_id_map.id_to_entity()[&CENTER]);
            },
        );
    }

    #[ignore = "Pathfinding where from_pos == to_pos just shouldn't happen right now"]
    #[test]
    fn find_path_to_position_in_self_best_route_is_to_just_ignore_gates() {
        let mut universe = UniverseSaveData::default();
        universe.sectors.add(CENTER);
        universe.sectors.add(RIGHT);

        universe.gate_pairs.add(
            LocalHexPosition::new(CENTER, Vec2::X * 1000.0),
            LocalHexPosition::new(RIGHT, Vec2::NEG_X),
        );
        universe.gate_pairs.add(
            LocalHexPosition::new(RIGHT, Vec2::X),
            LocalHexPosition::new(CENTER, Vec2::NEG_X * 1000.0),
        );

        let mut app = universe.build_test_app();
        let world = app.world_mut();

        world.run_system_once(
            move |sectors: Query<&Sector>,
                  transforms: Query<&SimulationTransform>,
                  sector_id_map: Res<SectorIdMap>| {
                let result = find_path(
                    &sectors,
                    &transforms,
                    sector_id_map.id_to_entity()[&CENTER],
                    Vec2::X,
                    sector_id_map.id_to_entity()[&CENTER],
                    Some(Vec2::NEG_X),
                )
                .unwrap();

                assert_eq!(result.len(), 0);
            },
        );
    }
}

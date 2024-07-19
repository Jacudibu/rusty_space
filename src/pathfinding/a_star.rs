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
) -> Option<Vec<PathElement>> {
    let mut open = BinaryHeap::new();
    let mut costs: HashMap<PathElement, u32> = HashMap::new();

    for (sector, gate_pair) in &sectors.get(from.into()).unwrap().gates {
        let cost_to_gate =
            cost(sectors, gate_positions, from, from_position, *sector).unwrap() - GATE_COST;
        let this = PathElement::new(*sector, *gate_pair);
        costs.insert(this, cost_to_gate);
        open.push(SearchNode {
            sector: *sector,
            gate_pair: *gate_pair,
            cost: cost_to_gate,
        });
    }

    // <Next, Previous>
    let mut came_from: HashMap<PathElement, PathElement> = HashMap::new();

    while let Some(node) = open.pop() {
        if node.sector == to {
            // TODO: multiple paths may lead to the same goal, and this might not be the best yet,
            //       especially if we want to move to a position further away from the gate -
            //       continue until all nodes have a bigger cost than the current best + distance to target_position
            return Some(reconstruct_path(&came_from, node));
        }

        let current = PathElement::new(node.sector, node.gate_pair);
        let current_cost = costs[&current];
        for (sector, gate_pair) in &sectors.get(node.sector.into()).unwrap().gates {
            let gate_pos = gate_positions
                .get(node.gate_pair.to.into())
                .unwrap()
                .translation;

            let Some(cost) = cost(sectors, gate_positions, node.sector, gate_pos, *sector) else {
                // Technically this never happens... yet. Maybe once we have initial sector fog of war, though.
                continue;
            };

            let neighbor = PathElement::new(*sector, *gate_pair);
            let neighbor_cost = current_cost + cost;
            if !costs.contains_key(&neighbor) || costs[&neighbor] > neighbor_cost {
                came_from.insert(neighbor, current);
                costs.insert(neighbor, neighbor_cost);
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
    to: SectorEntity,
) -> Option<u32> {
    if from_sector == to {
        return Some(0);
    }

    let a = sectors.get(from_sector.into()).unwrap();

    a.gates.get(&to).map(|gate| {
        let to_gate = gate_positions.get(gate.from.into()).unwrap();
        from_pos_in_sector
            .distance_squared(to_gate.translation)
            .abs() as u32
            + GATE_COST
    })
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
                let result = find_path(
                    &sectors,
                    &transforms,
                    sector_id_map.id_to_entity()[&LEFT2],
                    Vec2::ZERO,
                    sector_id_map.id_to_entity()[&RIGHT2],
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
}

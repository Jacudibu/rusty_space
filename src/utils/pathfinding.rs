use std::cmp::Ordering;
use std::collections::BinaryHeap;

use bevy::prelude::{Query, Transform, Vec3};
use bevy::utils::HashMap;

use crate::components::{GatePairInSector, Sector};
use crate::utils::SectorEntity;

pub struct PathElement {
    pub exit_sector: SectorEntity,
    pub gate_pair: GatePairInSector,
}

/// Returns the fastest gate-path between `from` and `to`.   
pub fn find_path(
    sectors: &Query<&Sector>,
    gate_positions: &Query<&Transform>,
    from: SectorEntity,
    from_position: Vec3,
    to: SectorEntity,
) -> Option<Vec<PathElement>> {
    a_star(sectors, gate_positions, from, from_position, to)
}

struct SearchNode {
    sector: SectorEntity,
    gate_pair: GatePairInSector,
    cost: u32,
}

const GATE_COST: u32 = 200;

impl Eq for SearchNode {}
impl PartialEq for SearchNode {
    fn eq(&self, other: &Self) -> bool {
        self.sector == other.sector && self.gate_pair == other.gate_pair
    }
}

impl PartialOrd for SearchNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SearchNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}

fn cost(
    sectors: &Query<&Sector>,
    gate_positions: &Query<&Transform>,
    from_sector: SectorEntity,
    from_pos_in_sector: Vec3,
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
    came_from: &HashMap<(SectorEntity, GatePairInSector), (SectorEntity, GatePairInSector)>,
    end: SearchNode,
) -> Vec<PathElement> {
    let mut path: Vec<PathElement> = std::iter::successors(
        Some(PathElement {
            exit_sector: end.sector,
            gate_pair: end.gate_pair,
        }),
        move |current| {
            came_from
                .get(&(current.exit_sector, current.gate_pair))
                .map(|current| PathElement {
                    exit_sector: current.0,
                    gate_pair: current.1,
                })
        },
    )
    .collect();
    path.reverse();
    path
}

fn a_star(
    sectors: &Query<&Sector>,
    gate_positions: &Query<&Transform>,
    from: SectorEntity,
    from_position: Vec3,
    to: SectorEntity,
) -> Option<Vec<PathElement>> {
    let mut open = BinaryHeap::new();
    let mut costs: HashMap<(SectorEntity, GatePairInSector), u32> = HashMap::new();
    // <(NextSector, EnterGatePair), (PreviousSector, PreviousGatePair)>
    let mut came_from: HashMap<(SectorEntity, GatePairInSector), (SectorEntity, GatePairInSector)> =
        HashMap::new();

    for (sector, gate_pair) in &sectors.get(from.into()).unwrap().gates {
        let cost_to_gate =
            cost(sectors, gate_positions, from, from_position, *sector).unwrap() - GATE_COST;
        let this = (*sector, *gate_pair);
        costs.insert(this, cost_to_gate);
        open.push(SearchNode {
            sector: *sector,
            gate_pair: *gate_pair,
            cost: cost_to_gate,
        });
        //came_from.insert(*sector, (from, *gate_pair));
    }

    while let Some(node) = open.pop() {
        if node.sector == to {
            // TODO: multiple paths may lead to the same goal, and this might not be the best yet
            //       if we want to move to a position further away from the gate -
            //       continue until all nodes have a bigger cost than the current best + distance to gate
            return Some(reconstruct_path(&came_from, node));
        }

        let current_cost = costs[&(node.sector, node.gate_pair)];
        for (sector, gate_pair) in &sectors.get(node.sector.into()).unwrap().gates {
            let gate_pos = gate_positions
                .get(node.gate_pair.to.into())
                .unwrap()
                .translation;

            let Some(cost) = cost(sectors, gate_positions, node.sector, gate_pos, *sector) else {
                // Technically this never happens... yet. Maybe once we have initial sector fog of war, though.
                continue;
            };

            let neighbor = (*sector, *gate_pair);
            let neighbor_cost = current_cost + cost;
            if !costs.contains_key(&neighbor) || costs[&neighbor] > neighbor_cost {
                came_from.insert(neighbor, (node.sector, node.gate_pair));
                costs.insert(neighbor, neighbor_cost);
                open.push(SearchNode {
                    sector: neighbor.0,
                    gate_pair: neighbor.1,
                    cost: neighbor_cost,
                })
            }
        }
    }

    None
}

#[cfg(test)]
mod test {
    use crate::components::Sector;
    use crate::universe_builder::gate_builder::HexPosition;
    use crate::universe_builder::sector_builder::HexToSectorEntityMap;
    use crate::universe_builder::UniverseBuilder;
    use crate::utils::pathfinding::find_path;
    use bevy::ecs::system::RunSystemOnce;
    use bevy::prelude::{Query, Res, Transform, Vec2, Vec3};
    use hexx::Hex;

    const LEFT2: Hex = Hex::new(-2, 0);
    const LEFT: Hex = Hex::new(-1, 0);
    const CENTER: Hex = Hex::new(0, 0);
    const RIGHT: Hex = Hex::new(1, 0);
    const RIGHT2: Hex = Hex::new(2, 0);

    #[test]
    fn find_path_to_neighbor() {
        let mut universe_builder = UniverseBuilder::default();
        universe_builder.sectors.add(CENTER);
        universe_builder.sectors.add(RIGHT);
        universe_builder.gates.add(
            HexPosition::new(CENTER, Vec2::ZERO),
            HexPosition::new(RIGHT, Vec2::ZERO),
        );

        let mut app = universe_builder.build_test_app();
        let world = app.world_mut();

        world.run_system_once(
            |sectors: Query<&Sector>,
             transforms: Query<&Transform>,
             hex_to_sector: Res<HexToSectorEntityMap>| {
                let result = find_path(
                    &sectors,
                    &transforms,
                    hex_to_sector.map[&CENTER],
                    Vec3::ZERO,
                    hex_to_sector.map[&RIGHT],
                )
                .unwrap();

                assert_eq!(result.len(), 1);
                assert_eq!(result[0].exit_sector, hex_to_sector.map[&RIGHT]);
            },
        );
    }

    #[test]
    fn find_path_through_single_sector() {
        let mut universe_builder = UniverseBuilder::default();
        universe_builder.sectors.add(LEFT);
        universe_builder.sectors.add(CENTER);
        universe_builder.sectors.add(RIGHT);
        universe_builder.gates.add(
            HexPosition::new(LEFT, Vec2::NEG_X),
            HexPosition::new(CENTER, Vec2::NEG_X),
        );
        universe_builder.gates.add(
            HexPosition::new(CENTER, Vec2::X),
            HexPosition::new(RIGHT, Vec2::X),
        );

        let mut app = universe_builder.build_test_app();
        let world = app.world_mut();

        world.run_system_once(
            |sectors: Query<&Sector>,
             transforms: Query<&Transform>,
             hex_to_sector: Res<HexToSectorEntityMap>| {
                let result = find_path(
                    &sectors,
                    &transforms,
                    hex_to_sector.map[&LEFT],
                    Vec3::ZERO,
                    hex_to_sector.map[&RIGHT],
                )
                .unwrap();

                assert_eq!(result.len(), 2);
                assert_eq!(result[0].exit_sector, hex_to_sector.map[&CENTER]);
                assert_eq!(result[1].exit_sector, hex_to_sector.map[&RIGHT]);
            },
        );
    }

    #[test]
    fn find_path_through_multiple_sectors() {
        let mut universe_builder = UniverseBuilder::default();
        universe_builder.sectors.add(LEFT);
        universe_builder.sectors.add(LEFT2);
        universe_builder.sectors.add(CENTER);
        universe_builder.sectors.add(RIGHT);
        universe_builder.sectors.add(RIGHT2);
        universe_builder.gates.add(
            HexPosition::new(LEFT2, Vec2::X),
            HexPosition::new(LEFT, Vec2::NEG_X),
        );
        universe_builder.gates.add(
            HexPosition::new(LEFT, Vec2::X),
            HexPosition::new(CENTER, Vec2::NEG_X),
        );
        universe_builder.gates.add(
            HexPosition::new(CENTER, Vec2::X),
            HexPosition::new(RIGHT, Vec2::NEG_X),
        );
        universe_builder.gates.add(
            HexPosition::new(RIGHT, Vec2::X),
            HexPosition::new(RIGHT2, Vec2::NEG_X),
        );

        let mut app = universe_builder.build_test_app();
        let world = app.world_mut();

        world.run_system_once(
            |sectors: Query<&Sector>,
             transforms: Query<&Transform>,
             hex_to_sector: Res<HexToSectorEntityMap>| {
                let result = find_path(
                    &sectors,
                    &transforms,
                    hex_to_sector.map[&LEFT2],
                    Vec3::ZERO,
                    hex_to_sector.map[&RIGHT2],
                )
                .unwrap();

                assert_eq!(result.len(), 4);
                assert_eq!(result[0].exit_sector, hex_to_sector.map[&LEFT]);
                assert_eq!(result[1].exit_sector, hex_to_sector.map[&CENTER]);
                assert_eq!(result[2].exit_sector, hex_to_sector.map[&RIGHT]);
                assert_eq!(result[3].exit_sector, hex_to_sector.map[&RIGHT2]);
            },
        );
    }
}

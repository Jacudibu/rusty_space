use std::cmp::Ordering;
use std::collections::BinaryHeap;

use bevy::prelude::{Query, Transform, Vec3};
use bevy::utils::HashMap;

use crate::components::{GateEntity, GatePairInSector, Sector, SectorEntity};

pub struct PathElement {
    pub enter_gate: GateEntity,
    pub exit_sector: SectorEntity,
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

    let a = sectors.get(from_sector.get()).unwrap();

    a.gates.get(&to).map(|gate| {
        let to_gate = gate_positions.get(gate.from.get()).unwrap();
        from_pos_in_sector
            .distance_squared(to_gate.translation)
            .abs() as u32
            + GATE_COST
    })
}

fn reconstruct_path(
    came_from: &HashMap<SectorEntity, (SectorEntity, GatePairInSector)>,
    end: SearchNode,
) -> Vec<PathElement> {
    let mut path: Vec<PathElement> = std::iter::successors(
        Some(PathElement {
            exit_sector: end.sector,
            enter_gate: end.gate_pair.from,
        }),
        move |current| {
            came_from
                .get(&current.exit_sector)
                .map(|current| PathElement {
                    exit_sector: current.0,
                    enter_gate: current.1.from,
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

    for (sector, gate_pair) in &sectors.get(from.get()).unwrap().gates {
        let cost_to_gate =
            cost(sectors, gate_positions, from, from_position, *sector).unwrap() - GATE_COST;
        let this = (*sector, *gate_pair);
        costs.insert(this, cost_to_gate);
        open.push(SearchNode {
            sector: *sector,
            gate_pair: *gate_pair,
            cost: cost_to_gate,
        })
    }

    // <NextSector, (PreviousSector, Gate)>
    let mut came_from: HashMap<SectorEntity, (SectorEntity, GatePairInSector)> = HashMap::new();

    while let Some(node) = open.pop() {
        if node.sector == to {
            // TODO: multiple paths may lead to the same goal, and this might not be the best yet
            //       continue until all nodes have a bigger cost than the current best
            return Some(reconstruct_path(&came_from, node));
        }

        let current_cost = costs[&(node.sector, node.gate_pair)];
        for (sector, gate_pair) in &sectors.get(node.sector.get()).unwrap().gates {
            let gate_pos = gate_positions
                .get(node.gate_pair.to.get())
                .unwrap()
                .translation;

            let Some(cost) = cost(sectors, gate_positions, node.sector, gate_pos, *sector) else {
                // Technically this never happens... yet. Maybe once we have initial sector fog of war, though.
                continue;
            };

            let neighbor = (*sector, *gate_pair);
            let neighbor_cost = current_cost + cost;
            if !costs.contains_key(&neighbor) || costs[&neighbor] > neighbor_cost {
                came_from.insert(node.sector, neighbor);
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
    use bevy::prelude::{Entity, Vec2, Vec3, World};
    use hexx::Hex;

    use crate::gizmos::*;
    use crate::utils::pathfinding::find_path;

    fn add_sector(world: &mut World, pos: Hex, gates: Vec<(Hex, Entity)>) -> Entity {
        let sector = Sector::new(pos, Vec2::ZERO);
        world.spawn(sector).id()
    }

    fn add_gates(world: &mut World, gates: Vec<((Entity, Vec3), (Entity, Vec3))>) {
        let mut commands = world.commands();
        let mut query = world.query::<&mut Sector>();
        for (a, b) in gates {
            let [mut a_sector, mut b_sector] = query.get_many_mut(world, [a.0, b.0]).unwrap();
            todo!();
            //world.spawn(GateComponent {  })

            a_sector.add_gate(&mut commands, a.0, .., b.0, ..)
        }

        world.flush()
    }

    #[test]
    fn find_path_to_neighbor() {
        let mut world = World::default();
        let from = Hex::new(0, 0);
        let to = Hex::new(1, 0);

        let mock_entity = Entity::from_raw(0);

        add_sector(&mut world, from, vec![(to, mock_entity)]);
        add_sector(&mut world, to, vec![(from, mock_entity)]);

        let result = find_path(&all_sectors, from, to).unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].enter_sector, from);
    }

    #[test]
    fn find_path_across_corner() {
        let mut all_sectors = AllSectors::default();
        let center = Hex::ZERO;
        let right = Hex::new(1, 0);
        let top_right = Hex::new(0, 1);

        let mock_entity = Entity::from_raw(0);

        add_sector(&mut all_sectors, center, vec![(right, mock_entity)]);
        add_sector(
            &mut all_sectors,
            right,
            vec![(center, mock_entity), (top_right, mock_entity)],
        );
        add_sector(&mut all_sectors, top_right, vec![(right, mock_entity)]);

        let result = find_path(&all_sectors, center, top_right).unwrap();

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].enter_sector, center);
        assert_eq!(result[1].enter_sector, right);
    }
}

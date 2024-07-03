use crate::sectors::sector_entity::SectorEntity;
use crate::sectors::{GateEntity, GateId, Sector};
use bevy::prelude::{Entity, Query};
use bevy::utils::HashMap;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub struct PathElement {
    pub enter_sector: SectorEntity,
    pub enter_gate_entity: Entity,
    pub exit_sector: SectorEntity,
    pub exit_gate: GateId,
}

/// Returns the fastest gate-path between `from` and `to`.   
pub fn find_path(
    sectors: &Query<&Sector>,
    from: SectorEntity,
    to: SectorEntity,
) -> Option<Vec<PathElement>> {
    a_star(sectors, from, to)
}

struct SearchNode {
    sector: SectorEntity,
    enter_at_gate: GateEntity,
    cost: u32,
}

const GATE_COST: u32 = 200;

impl Eq for SearchNode {}
impl PartialEq for SearchNode {
    fn eq(&self, other: &Self) -> bool {
        self.sector == other.sector && self.enter_at_gate == other.enter_at_gate
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

fn cost(sectors: &Query<&Sector>, from: Entity, to: Entity) -> Option<u32> {
    if from == to {
        return Some(0);
    }

    let a = sectors.get(from).unwrap();

    if let Some(gate) = a.gates.get(&to) {
        // TODO: distance to gate as cost
        Some(GATE_COST)
    } else {
        None
    }
}

// -> Which gate to use, where does it lead?

fn reconstruct_path(
    sectors: &Query<&Sector>,
    // <(Sector, Gate), NextSector>
    came_from: &HashMap<(SectorEntity, GateEntity), SectorEntity>,
    end: SearchNode,
) -> Vec<PathElement> {
    let exit_sector = sectors.get(end.sector).unwrap();
    let origin = came_from[&(end.sector, end.enter_at_gate)];

    let mut path: Vec<_> = std::iter::successors(Some(end), move |&current| {
        came_from
            .get(&(current.sector, current.enter_at_gate))
            .copied()
    })
    .reverse()
    .collect();

    Vec::new()
}

fn a_star(
    sectors: &Query<&Sector>,
    from: SectorEntity,
    to: SectorEntity,
) -> Option<Vec<PathElement>> {
    // TODO: startnode is a vec of all start sector gates
    let mut open = BinaryHeap::new();
    let mut costs: HashMap<(SectorEntity, GateEntity), u32> = HashMap::new();

    for (sector, gate_pair) in &sectors.get(from).unwrap().gates {
        let cost_to_gate = cost(sectors, from, *sector).unwrap() - GATE_COST;
        let this = (*sector, gate_pair.from);
        costs.insert(this, cost_to_gate);
        open.push(SearchNode {
            sector: *sector,
            enter_at_gate: gate_pair.to,
            cost: cost_to_gate,
        })
    }

    // <(Sector, Gate), NextSector>
    let mut came_from: HashMap<(Entity, Entity), Entity> = HashMap::new();

    while let Some(node) = open.pop() {
        if node.sector == to {
            // TODO: multiple paths may lead to the same goal, and this might not be the best yet
            //       continue until all nodes have a bigger cost than the current best
            return Some(reconstruct_path(sectors, &came_from, node));
        }

        let current_cost = costs[&(node.sector, node.enter_at_gate)];
        for (sector, gate_pair) in &sectors.get(node.sector).unwrap().gates {
            let Some(cost) = cost(sectors, node.sector, *sector) else {
                // No gate found, with how we traverse this, this should never happen?
                // TODO: Needs some testing, could optimize out the Option if correct
                continue;
            };

            let neighbor = (*sector, gate_pair.from);
            let neighbor_cost = current_cost + cost;
            if !costs.contains_key(&neighbor) || costs[&neighbor] > neighbor_cost {
                came_from.insert(neighbor, node.sector);
                costs.insert(neighbor, neighbor_cost);
                open.push(SearchNode {
                    sector: *sector,
                    enter_at_gate: gate_pair.to,
                    cost: neighbor_cost,
                })
            }
        }
    }

    None
}

#[cfg(test)]
mod test {
    use crate::sectors::pathfinding::find_path;
    use crate::sectors::*;
    use bevy::prelude::{Entity, Vec2, Vec3, World};
    use hexx::Hex;

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

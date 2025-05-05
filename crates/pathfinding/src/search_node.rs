use common::components::GatePairInSector;
use common::constants;
use common::types::entity_wrappers::SectorEntity;
use std::cmp::Ordering;

pub struct SearchNode {
    pub sector: SectorEntity,
    pub gate_pair: GatePairInSector,
    pub cost: u32,
}

// TODO: Remove this constant and use something in relation to ship speed instead.
//       Slow ships should be more inclined to use gates, whereas super fast ships might prefer skipping a gate.
pub const GATE_COST: u32 = (constants::SECTOR_SIZE * 3.0) as u32;

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
        // Since we use this in a binary heap, everything needs to be inverted
        other.cost.cmp(&self.cost).then_with(|| {
            other.sector.cmp(&self.sector).then_with(|| {
                other
                    .gate_pair
                    .from
                    .cmp(&self.gate_pair.from)
                    .then_with(|| other.gate_pair.to.cmp(&self.gate_pair.to))
            })
        })
    }
}

use crate::components::GatePairInSector;
use crate::utils::SectorEntity;
use std::cmp::Ordering;

pub struct SearchNode {
    pub sector: SectorEntity,
    pub gate_pair: GatePairInSector,
    pub cost: u32,
}

pub const GATE_COST: u32 = 200;

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

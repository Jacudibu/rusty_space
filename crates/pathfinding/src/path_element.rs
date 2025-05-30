use common::components::GatePairInSector;
use common::types::entity_wrappers::SectorEntity;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub struct PathElement {
    pub exit_sector: SectorEntity,
    pub gate_pair: GatePairInSector,
}

impl PathElement {
    pub fn new(exit_sector: SectorEntity, gate_pair: GatePairInSector) -> Self {
        Self {
            exit_sector,
            gate_pair,
        }
    }
}

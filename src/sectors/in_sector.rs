use crate::sectors::SectorId;
use bevy::prelude::Component;

/// Component for entities inside sectors.
///
/// These are managed by [SectorData], so if you feel a need to manually add or remove them,
/// you should probably use the respective functions there.
#[derive(Component, PartialEq, Eq)]
pub struct InSector {
    pub(in crate::sectors) sector: SectorId,
}

impl InSector {
    pub fn get(&self) -> SectorId {
        self.sector
    }
}

impl PartialEq<SectorId> for InSector {
    fn eq(&self, other: &SectorId) -> bool {
        &self.sector == other
    }
}

impl PartialEq<SectorId> for &InSector {
    fn eq(&self, other: &SectorId) -> bool {
        &self.sector == other
    }
}

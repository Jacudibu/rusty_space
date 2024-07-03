use bevy::prelude::{Component, Entity};

/// Component for entities inside sectors.
///
/// These are managed by [SectorData], so if you feel a need to manually add or remove them,
/// you should probably use the respective functions there.
#[derive(Component, PartialEq, Eq)]
pub struct InSector {
    pub(in crate::sectors) sector: Entity,
}

impl InSector {
    pub fn get(&self) -> Entity {
        self.sector
    }
}

impl PartialEq<Entity> for InSector {
    fn eq(&self, other: &Entity) -> bool {
        &self.sector == other
    }
}

impl PartialEq<Entity> for &InSector {
    fn eq(&self, other: &Entity) -> bool {
        &self.sector == other
    }
}

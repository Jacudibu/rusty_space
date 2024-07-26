use crate::utils::SectorEntity;
use bevy::prelude::{Commands, Component, Entity};

/// Component for entities inside sectors.
///
/// These are managed by [Sector], so if you feel a need to manually add or remove them,
/// you should probably use the respective functions there.
#[derive(Component, PartialEq, Eq)]
pub struct InSector {
    pub(crate) sector: SectorEntity,
}

impl InSector {
    pub fn get(&self) -> SectorEntity {
        self.sector
    }

    /// Adds an [`InSector`] component to `entity`, which links to the provided `sector_entity`.
    pub fn add_component(commands: &mut Commands, sector_entity: SectorEntity, entity: Entity) {
        commands.entity(entity).insert(InSector {
            sector: sector_entity,
        });
    }
}

impl PartialEq<SectorEntity> for InSector {
    fn eq(&self, other: &SectorEntity) -> bool {
        &self.sector == other
    }
}

impl PartialEq<SectorEntity> for &InSector {
    fn eq(&self, other: &SectorEntity) -> bool {
        &self.sector == other
    }
}

impl From<&InSector> for Entity {
    fn from(value: &InSector) -> Self {
        value.sector.into()
    }
}

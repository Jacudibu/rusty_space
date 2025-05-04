use crate::session_data::ship_configs::ShipConfigId;
use crate::types::persistent_entity_id::{ComponentWithPersistentId, PersistentShipId};
use bevy::prelude::Component;

/// Marker Component for Ships
#[derive(Component)]
#[component(immutable)]
pub struct Ship {
    id: PersistentShipId,
    config_id: ShipConfigId,
}

impl Ship {
    #[inline]
    pub fn new(id: PersistentShipId, config_id: ShipConfigId) -> Self {
        Self { id, config_id }
    }

    #[inline]
    pub fn config_id(&self) -> ShipConfigId {
        self.config_id
    }
}

impl ComponentWithPersistentId<Ship> for Ship {
    #[inline]
    fn id(&self) -> PersistentShipId {
        self.id
    }
}

use crate::persistence::{ComponentWithPersistentId, PersistentShipId};
use crate::session_data::ShipConfigId;
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

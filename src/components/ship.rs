use crate::persistence::{ComponentWithPersistentId, PersistentShipId};
use bevy::prelude::Component;

/// Marker Component for Ships
#[derive(Component)]
pub struct Ship {
    id: PersistentShipId,
}

impl Ship {
    #[inline]
    pub fn new(id: PersistentShipId) -> Self {
        Self { id }
    }
}

impl ComponentWithPersistentId<Ship> for Ship {
    #[inline]
    fn id(&self) -> PersistentShipId {
        self.id
    }
}

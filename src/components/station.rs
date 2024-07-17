use crate::persistence::{ComponentWithPersistentId, PersistentStationId, TypedPersistentEntityId};
use bevy::prelude::Component;

/// Marker Component for Stations
#[derive(Component)]
pub struct Station {
    pub id: PersistentStationId,
}

impl ComponentWithPersistentId<Station> for Station {
    #[inline]
    fn id(&self) -> TypedPersistentEntityId<Station> {
        self.id
    }
}

impl Station {
    #[inline]
    pub fn new(id: TypedPersistentEntityId<Station>) -> Self {
        Self { id }
    }
}

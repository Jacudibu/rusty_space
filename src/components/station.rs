use crate::persistence::{PersistentStationId, TypedPersistentEntityId};
use bevy::prelude::Component;

/// Marker Component for Stations
#[derive(Component)]
pub struct Station {
    pub id: PersistentStationId,
}

impl Station {
    pub fn new(id: TypedPersistentEntityId<Station>) -> Self {
        Self { id }
    }
}

use crate::persistence::{ComponentWithPersistentId, PersistentStationId, TypedPersistentEntityId};
use crate::utils::ConstructionSiteEntity;
use bevy::prelude::Component;

/// Marker Component for Stations
#[derive(Component)]
pub struct StationComponent {
    /// The PersistentEntityId assigned to this Station.
    pub id: PersistentStationId,

    /// The Entity of this station's active build site, if there is any.
    pub construction_site: Option<ConstructionSiteEntity>,
}

impl ComponentWithPersistentId<StationComponent> for StationComponent {
    #[inline]
    fn id(&self) -> TypedPersistentEntityId<StationComponent> {
        self.id
    }
}

impl StationComponent {
    #[inline]
    pub fn new(
        id: TypedPersistentEntityId<StationComponent>,
        construction_site: Option<ConstructionSiteEntity>,
    ) -> Self {
        Self {
            id,
            construction_site,
        }
    }
}

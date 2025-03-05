use crate::persistence::{
    ComponentWithPersistentId, PersistentBuildSiteId, PersistentStationId, TypedPersistentEntityId,
};
use bevy::prelude::Component;

/// Marker Component for Stations
#[derive(Component)]
pub struct Station {
    /// The PersistentEntityId assigned to this Station.
    pub id: PersistentStationId,

    /// The PersistentEntityId of this station's active build site, if there is any.
    pub build_site_id: Option<PersistentBuildSiteId>,
}

impl ComponentWithPersistentId<Station> for Station {
    #[inline]
    fn id(&self) -> TypedPersistentEntityId<Station> {
        self.id
    }
}

impl Station {
    #[inline]
    pub fn new(
        id: TypedPersistentEntityId<Station>,
        build_site_id: Option<PersistentBuildSiteId>,
    ) -> Self {
        Self { id, build_site_id }
    }
}

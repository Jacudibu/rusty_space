use crate::persistence::{ComponentWithPersistentId, PersistentStationId, TypedPersistentEntityId};
use crate::utils::ConstructionSiteEntity;
use bevy::prelude::Component;

/// Marker Component for immovable, player-constructed objects in space.
/// Depending on the modules built inside them, they are usually used for ship construction or resource processing.
#[derive(Component)]
pub struct Station {
    /// The PersistentEntityId assigned to this Station.
    pub id: PersistentStationId,

    /// The Entity of this station's active build site, if there is any.
    pub construction_site: Option<ConstructionSiteEntity>,
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
        construction_site: Option<ConstructionSiteEntity>,
    ) -> Self {
        Self {
            id,
            construction_site,
        }
    }
}

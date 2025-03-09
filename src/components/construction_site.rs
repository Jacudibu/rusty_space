use crate::game_data::ConstructableModuleId;
use crate::persistence::{PersistentConstructionSiteId, PersistentStationId};
use bevy::prelude::Component;

/// Marker component for ConstructionSites.
/// These are always directly owned by a station, but have their own inventory and buy orders.
/// Build Sites disappear once everything has been built.
#[derive(Component)]
pub struct ConstructionSite {
    /// The PersistentEntityId of this ConstructionSite.
    pub id: PersistentConstructionSiteId,

    /// The PersistentEntityId of the Station owning this ConstructionSite.
    pub station_id: PersistentStationId,

    /// A queue of the modules which still need to be built.
    pub build_order: Vec<ConstructableModuleId>,

    /// The total amount of build power that's already been applied to the first element inside our [build_order].
    pub current_build_progress: f32,
}

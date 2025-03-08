use crate::game_data::BuildableModuleId;
use crate::persistence::{PersistentBuildSiteId, PersistentStationId};
use bevy::prelude::Component;

/// Marker component for BuildSites.
/// These are always directly owned by a station, but have their own inventory and buy orders.
/// Build Sites disappear once everything has been built.
#[derive(Component)]
pub struct BuildSite {
    /// The PersistentEntityId of this BuildSite.
    pub id: PersistentBuildSiteId,

    /// The PersistentEntityId of the Station owning this BuildSite.
    pub station_id: PersistentStationId,

    /// A queue of the modules which still need to be built.
    pub build_order: Vec<BuildableModuleId>,

    /// The total amount of build power that's already been applied to the first element inside our [build_order].
    pub current_build_progress: f32,
}

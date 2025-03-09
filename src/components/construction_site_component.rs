use crate::game_data::{ConstructableModuleId, ItemId};
use crate::persistence::{PersistentConstructionSiteId, PersistentStationId};
use bevy::prelude::Component;

/// Marker component for ConstructionSites.
/// These are always directly owned by a station, but have their own inventory and buy orders.
/// Build Sites disappear once everything has been built.
#[derive(Component)]
pub struct ConstructionSiteComponent {
    /// The PersistentEntityId of this ConstructionSite.
    pub id: PersistentConstructionSiteId,

    /// The PersistentEntityId of the Station owning this ConstructionSite.
    pub station_id: PersistentStationId,

    /// A queue of the modules which still need to be built. There'll always be at least one element within this queue.
    pub build_order: Vec<ConstructableModuleId>,

    /// The total amount of build power that's already been applied to the first element inside our [build_order].
    pub current_build_progress: f32,

    /// The Current Status of this Construction Site, indicating why things aren't progressing.
    pub status: ConstructionSiteStatus,

    /// How many construction ships are currently actively working on this site.
    pub construction_ship_count: u32,

    /// The total construction power of all construction ships currently working on this site.
    /// A higher [construction_ship_count] means less of this will be applied due to inefficiencies.
    pub total_construction_power: u32,
}

pub enum ConstructionSiteStatus {
    Ok,
    MissingMaterials(Vec<ItemId>),
    MissingBuilders,
}

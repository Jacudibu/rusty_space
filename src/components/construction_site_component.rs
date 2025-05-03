use crate::game_data::{ConstructableModuleId, ItemId};
use crate::persistence::PersistentConstructionSiteId;
use crate::utils::{ShipEntity, StationEntity};
use bevy::platform::collections::HashSet;
use bevy::prelude::Component;

/// Marker component for ConstructionSites.
/// These are always directly owned by a station, but have their own inventory and buy orders.
/// Build Sites disappear once everything has been built.
#[derive(Component)]
pub struct ConstructionSiteComponent {
    /// The PersistentEntityId of this ConstructionSite.
    pub id: PersistentConstructionSiteId,

    /// The Station owning this ConstructionSite.
    pub station: StationEntity,

    /// A queue of the modules which still need to be built. There'll always be at least one element within this queue.
    pub build_order: Vec<ConstructableModuleId>,

    /// The total amount of build power that's already been applied to the first element inside our [build_order].
    pub current_build_progress: f32,

    /// Reaching this amount of progress will trigger the next construction step.
    pub progress_until_next_step: f32,

    /// Used to identify which materials will be consumed once we exceed the progress required for the next step.
    pub next_construction_step: usize,

    /// The Current Status of this Construction Site, indicating why things aren't progressing.
    pub status: ConstructionSiteStatus,

    /// The construction ships which are currently actively working on this site.
    pub construction_ships: HashSet<ShipEntity>,

    /// The total construction power of all construction ships currently working on this site.
    /// A higher [construction_ship_count] means less of this will be applied due to inefficiencies.
    pub total_build_power_of_ships: u32,
}

pub enum ConstructionSiteStatus {
    Ok,
    MissingMaterials(Vec<ItemId>),
    MissingBuilders,
}

use crate::game_data::{ConstructableModuleId, ItemId};
use crate::types::entity_wrappers::{ShipEntity, StationEntity};
use crate::types::persistent_entity_id::PersistentConstructionSiteId;
use bevy::prelude::Component;
use std::collections::HashMap;

/// Marker component for ConstructionSites.
/// These are always directly owned by a station, but have their own inventory and buy orders.
/// Build Sites despawn once everything has been built.
#[derive(Component)]
pub struct ConstructionSite {
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

    /// The construction ships which are currently actively working on this site and their individual strength.
    pub construction_ships: HashMap<ShipEntity, u32>,

    /// The total construction power of all construction ships currently working on this site.
    /// A higher construction ship count means less of this will be applied due to inefficiencies.
    /// Do not change this value independently of [construction_ships]
    pub total_build_power_of_ships: u32,
}

pub enum ConstructionSiteStatus {
    Ok,
    MissingMaterials(Vec<ItemId>),
    MissingBuilders,
}

use crate::game_data::ShipyardModuleId;
use crate::session_data::ShipConfigId;
use crate::simulation::prelude::SimulationTimestamp;
use bevy::prelude::Component;
use bevy::utils::HashMap;

/// A component on [StationEntity]s which keeps track of ship building modules and requests.
#[derive(Component)]
pub struct ShipyardComponent {
    pub modules: HashMap<ShipyardModuleId, ShipyardModule>,

    /// The ships which are queued to be built in the near future at this shipyard.
    pub queue: Vec<ShipConfigId>,
}

pub struct ShipyardModule {
    /// How many instances of this module exist
    pub amount: u32,

    /// A list of ship orders which are actively being built right now
    pub active: Vec<OngoingShipConstructionOrder>,
}

impl Default for ShipyardModule {
    fn default() -> Self {
        Self {
            amount: 1,
            active: Default::default(),
        }
    }
}

/// Holds data detailing one specific ship that's currently being worked on within a shipyard.
pub struct OngoingShipConstructionOrder {
    /// The [ShipConfigId] of the built ship.
    pub ship_config: ShipConfigId,

    /// A timestamp when this order is supposed to be completed.
    pub finished_at: SimulationTimestamp,
}

use crate::utils::SimulationTimestamp;
use bevy::prelude::Component;

// TODO: Check if idle needs to be generic on ShipBehavior to guarantee system parallelism.
//       Alternatively, maybe just using commands.insert_bulk() with a new idle could be cheaper?
#[derive(Component)]
pub struct Idle {
    pub next_update: SimulationTimestamp,
}

impl Default for Idle {
    fn default() -> Self {
        Self {
            next_update: SimulationTimestamp::MIN,
        }
    }
}

use bevy::prelude::{Res, ResMut, Resource, Time, Virtual};
use std::time::Duration;

/// Used to schedule things that will happen at a specific time.
pub type SimulationSeconds = u32;

/// Keeps track of the simulation in seconds. Used to process anything that's supposed to happen at a specific time.
#[derive(Resource)]
pub struct SimulationTime {
    /// The total Duration since the simulation has started.
    total: Duration,
}

impl Default for SimulationTime {
    fn default() -> Self {
        SimulationTime {
            total: Duration::ZERO,
        }
    }
}

impl SimulationTime {
    #[inline]
    fn advance(&mut self, delta: Duration) {
        self.total += delta;
    }

    #[inline]
    pub fn seconds(&self) -> SimulationSeconds {
        self.total.as_secs() as SimulationSeconds // using the full u64 would be overkill
    }
}

/// Should always run **after** [bevy::time::TimeSystem]
pub fn update(mut simulation_time: ResMut<SimulationTime>, bevy_time: Res<Time<Virtual>>) {
    simulation_time.advance(bevy_time.delta());
}

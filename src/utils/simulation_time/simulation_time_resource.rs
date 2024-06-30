use crate::utils::simulation_time::simulation_timestamp::CurrentSimulationTimestamp;
use crate::utils::Milliseconds;
use bevy::prelude::Resource;
use std::time::Duration;

/// Keeps track of the simulation in seconds. Used to process anything that's supposed to happen at a specific time.
/// Use [SimulationTimestamp] to schedule when things are supposed to happen at (or shortly past) a specific point in time.
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
    pub(in crate::utils::simulation_time) fn advance(&mut self, delta: Duration) {
        self.total += delta;
    }

    /// Returns the [CurrentSimulationTimestamp], which can then be used to create or interact with [SimulationTimestamp]s for task scheduling.
    #[inline]
    pub fn now(&self) -> CurrentSimulationTimestamp {
        CurrentSimulationTimestamp::from(self.total.as_millis() as Milliseconds)
    }
}

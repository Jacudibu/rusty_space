use std::time::Duration;

use bevy::prelude::Resource;

use crate::simulation::prelude::{CurrentSimulationTimestamp, Milliseconds};

/// Keeps track of the simulation in seconds. Used to process anything that's supposed to happen at a specific time.
/// Use [SimulationTimestamp] to schedule when things are supposed to happen at (or shortly past) a specific point in time.
#[derive(Resource)]
pub struct SimulationTime {
    /// The total Duration since the simulation has started.
    total: Duration,

    /// The current tick. Increases by one for every time the FixedUpdate Schedule is run.
    tick: u32,
}

impl Default for SimulationTime {
    fn default() -> Self {
        SimulationTime {
            total: Duration::ZERO,
            tick: 0,
        }
    }
}

impl SimulationTime {
    #[inline]
    pub(crate) fn advance(&mut self, delta: Duration) {
        self.total += delta;
        self.tick += 1;
    }

    /// Returns the [CurrentSimulationTimestamp], which can then be used to create or interact with [SimulationTimestamp]s for task scheduling.
    #[inline]
    pub fn now(&self) -> CurrentSimulationTimestamp {
        CurrentSimulationTimestamp::from(self.total.as_millis() as Milliseconds)
    }

    /// Returns the current tick - a counter for how many FixedUpdate schedules have been run in total within this simulation.
    #[inline]
    #[allow(dead_code)]
    pub fn tick(&self) -> u32 {
        self.tick
    }
}

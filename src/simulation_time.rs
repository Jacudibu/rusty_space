use bevy::prelude::{Res, ResMut, Resource, Time};
use std::time::Duration;

#[derive(Resource)]
pub struct SimulationTime {
    /// The total Duration since the simulation has started.
    total: Duration,
    /// How much time has passed since the last update.
    delta: Duration,
    /// How much time has passed since the last update as f32.
    delta_seconds: f32,
    /// Multiplier for delta time updates.
    scale: f32,
}

impl Default for SimulationTime {
    fn default() -> Self {
        SimulationTime {
            delta: Duration::ZERO,
            total: Duration::ZERO,
            delta_seconds: 0.0,
            scale: 1.0,
        }
    }
}

impl SimulationTime {
    fn advance(&mut self, delta: Duration) {
        self.delta = delta.mul_f32(self.scale);
        self.delta_seconds = self.delta.as_secs_f32();
        self.total += self.delta;
    }

    #[inline]
    pub fn delta_seconds(&self) -> f32 {
        self.delta_seconds
    }

    #[inline]
    pub fn total_seconds(&self) -> u32 {
        self.total.as_secs() as u32 // using the full u64 would be overkill
    }

    #[inline]
    pub fn scale(&self) -> f32 {
        self.scale
    }

    #[inline]
    pub fn elapsed_seconds_f32(&self) -> f32 {
        self.total.as_secs_f32()
    }
}

pub fn update(mut simulation_time: ResMut<SimulationTime>, real_time: Res<Time>) {
    simulation_time.advance(real_time.delta());
}

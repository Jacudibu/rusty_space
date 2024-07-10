use crate::constants;
use crate::utils::SimulationTimestamp;
use bevy::prelude::{Component, FloatExt, Transform, Vec3};

#[derive(Component)]
pub struct Asteroid {
    ore_max: u32,
    pub ore: u32,
    pub remaining_after_reservations: u32,

    /// Timestamp for this asteroids' next scheduled despawn or spawn event
    pub next_event_timestamp: SimulationTimestamp,
}

impl Asteroid {
    pub fn new(ore: u32, next_event_timestamp: SimulationTimestamp) -> Self {
        Self {
            ore,
            ore_max: ore,
            remaining_after_reservations: ore,
            next_event_timestamp,
        }
    }

    pub fn reset(&mut self, transform: &mut Transform) {
        self.ore = self.ore_max;
        self.remaining_after_reservations = self.ore_max;
        transform.scale = self.scale_depending_on_current_ore_volume();
    }

    pub fn scale_depending_on_current_ore_volume(&self) -> Vec3 {
        const MIN: f32 = 0.3;
        const MAX: f32 = 1.5;
        let t = self.ore as f32 / constants::ASTEROID_ORE_RANGE.end as f32;

        Vec3::splat(MIN.lerp(MAX, t))
    }
}

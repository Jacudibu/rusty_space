use crate::constants;
use crate::utils::{Milliseconds, SimulationTimestamp};
use bevy::prelude::{Component, FloatExt, Transform, Vec3};

#[derive(Component)]
pub struct Asteroid {
    ore_max: u32,
    pub ore: u32,
    pub remaining_after_reservations: u32,
    pub state: AsteroidState,
}

pub enum AsteroidState {
    Spawned { until: SimulationTimestamp },
    Despawned { until: SimulationTimestamp },
}

impl AsteroidState {
    pub fn toggle_and_add_milliseconds(&self, milliseconds: Milliseconds) -> Self {
        match self {
            AsteroidState::Spawned { mut until } => {
                until.add_milliseconds(milliseconds);
                AsteroidState::Despawned { until }
            }
            AsteroidState::Despawned { mut until } => {
                until.add_milliseconds(milliseconds);
                AsteroidState::Spawned { until }
            }
        }
    }

    pub fn timestamp(&self) -> SimulationTimestamp {
        match self {
            AsteroidState::Spawned { until } => *until,
            AsteroidState::Despawned { until } => *until,
        }
    }
}

impl Asteroid {
    pub fn new(ore: u32, state: AsteroidState) -> Self {
        Self {
            ore,
            ore_max: ore,
            remaining_after_reservations: ore,
            state,
        }
    }

    pub fn reset(&mut self, transform: &mut Transform) {
        self.ore = self.ore_max;
        self.remaining_after_reservations = self.ore_max;
        transform.scale = self.scale_depending_on_current_ore_volume();
    }

    /// Attempts to reserve the desired amount if possible, or less if there isn't as much left.
    /// ### Returns
    /// The actual amount which got reserved.
    pub fn try_to_reserve(&mut self, desired_amount: u32) -> u32 {
        let amount = desired_amount.min(self.remaining_after_reservations);
        self.remaining_after_reservations -= amount;
        amount
    }

    pub fn scale_depending_on_current_ore_volume(&self) -> Vec3 {
        const MIN: f32 = 0.3;
        const MAX: f32 = 1.5;
        let t = self.ore as f32 / constants::ASTEROID_ORE_RANGE.end as f32;

        Vec3::splat(MIN.lerp(MAX, t))
    }
}

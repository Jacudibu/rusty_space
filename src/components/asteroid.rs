use crate::constants;
use crate::persistence::{ComponentWithPersistentId, PersistentAsteroidId};
use crate::simulation::prelude::SimulationTimestamp;
use bevy::prelude::{Component, FloatExt};

#[derive(Component)]
pub struct Asteroid {
    id: PersistentAsteroidId,
    pub ore_max: u32,
    pub ore: u32,
    pub remaining_after_reservations: u32,
    pub despawn_timestamp: SimulationTimestamp,
}

impl ComponentWithPersistentId<Asteroid> for Asteroid {
    #[inline]
    fn id(&self) -> PersistentAsteroidId {
        self.id
    }
}

impl Asteroid {
    pub fn new(id: PersistentAsteroidId, ore: u32, despawn_timestamp: SimulationTimestamp) -> Self {
        Self {
            id,
            ore,
            ore_max: ore,
            remaining_after_reservations: ore,
            despawn_timestamp,
        }
    }

    /// Attempts to reserve the desired amount if possible, or less if there isn't as much left.
    /// ### Returns
    /// The actual amount which got reserved.
    pub fn try_to_reserve(&mut self, desired_amount: u32) -> u32 {
        let amount = desired_amount.min(self.remaining_after_reservations);
        self.remaining_after_reservations -= amount;
        amount
    }

    pub fn scale_depending_on_current_ore_volume(&self) -> f32 {
        const MIN: f32 = 0.3;
        const MAX: f32 = 1.5;
        let t = self.ore as f32 / constants::ASTEROID_ORE_RANGE.end as f32;

        MIN.lerp(MAX, t)
    }
}

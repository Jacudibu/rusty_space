use crate::constants;
use crate::game_data::{AsteroidDataId, ItemId};
use crate::simulation_time::SimulationTimestamp;
use crate::types::persistent_entity_id::{ComponentWithPersistentId, PersistentAsteroidId};
use bevy::prelude::{Component, FloatExt};

#[derive(Component)]
pub struct Asteroid {
    persistent_id: PersistentAsteroidId,
    manifest_id: AsteroidDataId,
    pub ore_max: u32,
    pub ore: u32,
    pub ore_item_id: ItemId,
    pub remaining_after_reservations: u32,
    pub despawn_timestamp: SimulationTimestamp,
}

impl ComponentWithPersistentId<Asteroid> for Asteroid {
    #[inline]
    fn id(&self) -> PersistentAsteroidId {
        self.persistent_id
    }
}

impl Asteroid {
    pub fn new(
        id: PersistentAsteroidId,
        manifest_id: AsteroidDataId,
        ore_item_id: ItemId,
        ore: u32,
        ore_max: u32,
        despawn_timestamp: SimulationTimestamp,
    ) -> Self {
        Self {
            persistent_id: id,
            manifest_id,
            ore,
            ore_max,
            ore_item_id,
            remaining_after_reservations: ore,
            despawn_timestamp,
        }
    }

    pub fn manifest_id(&self) -> AsteroidDataId {
        self.manifest_id
    }

    /// Attempts to reserve the desired amount if possible, or less if there isn't as much left.
    /// ### Returns
    /// The actual amount which got reserved.
    pub fn try_to_reserve(&mut self, desired_amount: u32) -> u32 {
        let amount = desired_amount.min(self.remaining_after_reservations);
        self.remaining_after_reservations -= amount;
        amount
    }

    /// Remove a reservation.
    pub fn unreserve(&mut self, amount: u32) {
        self.remaining_after_reservations += amount;
    }

    pub fn scale_depending_on_current_ore_volume(&self) -> f32 {
        const MIN: f32 = 0.3;
        const MAX: f32 = 1.5;
        let t = self.ore as f32 / constants::ASTEROID_ORE_RANGE.end as f32;

        MIN.lerp(MAX, t)
    }
}

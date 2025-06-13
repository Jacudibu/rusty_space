use crate::constants;
use crate::game_data::{AsteroidDataId, ItemId};
use crate::simulation_time::SimulationTimestamp;
use crate::types::persistent_entity_id::{ComponentWithPersistentId, PersistentAsteroidId};
use bevy::prelude::{Component, FloatExt};

/// Marker Component for Asteroids.
/// Asteroids are rocks that idly float through space and can be mined for the precious resources they may carry.
#[derive(Component)]
pub struct Asteroid {
    persistent_id: PersistentAsteroidId,
    manifest_id: AsteroidDataId,
    /// How much ore does this asteroid have when freshly spawned?
    pub ore_max: u32,
    /// How much ore is remaining in this asteroid
    pub ore_remaining: u32,
    /// Which item can be mined from this asteroid?
    pub ore_item_id: ItemId,
    /// When will this asteroid reach the edge of the sector and despawn?
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
            ore_remaining: ore,
            ore_max,
            ore_item_id,
            despawn_timestamp,
        }
    }

    pub fn manifest_id(&self) -> AsteroidDataId {
        self.manifest_id
    }

    pub fn scale_depending_on_current_ore_volume(&self) -> f32 {
        const MIN: f32 = 0.3;
        const MAX: f32 = 1.5;
        let t = self.ore_remaining as f32 / constants::ASTEROID_ORE_RANGE.end as f32;

        MIN.lerp(MAX, t)
    }
}

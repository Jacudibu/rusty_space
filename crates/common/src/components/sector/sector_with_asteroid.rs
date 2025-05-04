use crate::components::constant_velocity::ConstantVelocity;
use crate::components::{Asteroid, InSector};
use crate::constants;
use crate::game_data::{AsteroidDataId, ItemId};
use crate::simulation_time::SimulationTimestamp;
use crate::types::entity_wrappers::{AsteroidEntityWithTimestamp, SectorEntity};
use crate::types::persistent_entity_id::{ComponentWithPersistentId, PersistentAsteroidId};
use bevy::math::Vec2;
use bevy::platform::collections::HashMap;
use bevy::prelude::{Commands, Component};
use std::cmp::Ordering;
use std::collections::{BTreeSet, BinaryHeap};

/// A sector with this component features small asteroids floating through it.
#[derive(Component)]
pub struct SectorWithAsteroids {
    average_velocity: Vec2,

    /// The kind of materials which can be found in asteroids within this sector. These should never change during runtime.
    asteroid_materials: Vec<ItemId>,

    /// Contains individual collections of the "live" asteroids for all `asteroid_materials`
    pub asteroids: HashMap<ItemId, BTreeSet<AsteroidEntityWithTimestamp>>,

    /// Contains individual collections of the respawning asteroids for all `asteroid_materials`
    pub asteroid_respawns: HashMap<ItemId, BinaryHeap<std::cmp::Reverse<RespawningAsteroidData>>>,
}

impl SectorWithAsteroids {
    #[must_use]
    pub fn new(average_velocity: Vec2, asteroid_materials: Vec<ItemId>) -> Self {
        Self {
            average_velocity,
            asteroids: HashMap::from_iter(
                asteroid_materials
                    .iter()
                    .map(|id| (*id, Default::default())),
            ),
            asteroid_respawns: HashMap::from_iter(
                asteroid_materials
                    .iter()
                    .map(|id| (*id, Default::default())),
            ),
            asteroid_materials,
        }
    }

    #[inline]
    #[must_use]
    #[allow(dead_code)]
    pub fn average_velocity(&self) -> &Vec2 {
        &self.average_velocity
    }

    #[inline]
    #[must_use]
    pub fn asteroid_types(&self) -> &Vec<ItemId> {
        &self.asteroid_materials
    }

    /// How "Healthy" the asteroid field of this sector is... as in how many of its asteroids are currently spawned, in Range [0,1].
    #[must_use]
    pub fn remaining_percentage(&self, requested_material: &ItemId) -> f32 {
        let Some(asteroids) = self.asteroids.get(requested_material) else {
            return 0.0;
        };

        let spawned = asteroids.len();
        let despawned = self.asteroid_respawns.len();

        spawned as f32 / (spawned + despawned) as f32
    }

    /// Adds the given asteroid to this sector and inserts the [InSector] component to it.
    pub fn add_asteroid(
        &mut self,
        commands: &mut Commands,
        sector_entity: SectorEntity,
        entity: AsteroidEntityWithTimestamp,
        item_id: ItemId,
    ) {
        self.asteroids.get_mut(&item_id).unwrap().insert(entity);
        InSector::add_component(commands, sector_entity, entity.entity.into());
    }
}

#[derive(Copy, Clone)]
pub struct RespawningAsteroidData {
    pub id: PersistentAsteroidId,
    pub item_id: AsteroidDataId,
    pub ore_max: u32,
    pub local_respawn_position: Vec2,
    pub velocity: Vec2,
    pub angular_velocity: f32,
    pub timestamp: SimulationTimestamp,
}

impl RespawningAsteroidData {
    pub fn new(
        value: &Asteroid,
        velocity: &ConstantVelocity,
        local_respawn_position: Vec2,
    ) -> Self {
        Self {
            id: value.id(),
            item_id: value.manifest_id(),
            ore_max: value.ore_max,
            local_respawn_position,
            timestamp: value.despawn_timestamp + constants::ASTEROID_RESPAWN_TIME,
            velocity: velocity.velocity(),
            angular_velocity: velocity.sprite_rotation(),
        }
    }
}

impl Eq for RespawningAsteroidData {}

impl PartialEq<Self> for RespawningAsteroidData {
    fn eq(&self, other: &Self) -> bool {
        other.id == self.id
    }
}

impl PartialOrd<Self> for RespawningAsteroidData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RespawningAsteroidData {
    fn cmp(&self, other: &Self) -> Ordering {
        self.timestamp
            .cmp(&other.timestamp)
            .then_with(|| self.id.cmp(&other.id))
    }
}

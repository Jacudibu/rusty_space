use crate::components::{Asteroid, InSector};
use crate::constants;
use crate::game_data::ItemId;
use crate::persistence::{ComponentWithPersistentId, PersistentAsteroidId};
use crate::simulation::physics::ConstantVelocity;
use crate::simulation::prelude::SimulationTimestamp;
use crate::utils::{AsteroidEntityWithTimestamp, SectorEntity};
use bevy::math::Vec2;
use bevy::prelude::{Commands, Component};
use bevy::utils::HashMap;
use std::cmp::Ordering;
use std::collections::{BTreeSet, BinaryHeap};

/// A sector with this component features small asteroids floating through it.
#[derive(Component)]
pub struct SectorAsteroidComponent {
    average_velocity: Vec2,

    /// The kind of asteroids that can be found within this sector. These should never change during runtime.
    asteroid_types: Vec<ItemId>,

    /// Contains individual collections of the "live" asteroids for all `asteroid_types`
    pub asteroids: HashMap<ItemId, BTreeSet<AsteroidEntityWithTimestamp>>,

    /// Contains individual collections of the respawning asteroids for all `asteroid_types`
    pub asteroid_respawns: HashMap<ItemId, BinaryHeap<std::cmp::Reverse<RespawningAsteroidData>>>,
}

impl SectorAsteroidComponent {
    #[must_use]
    pub fn new(average_velocity: Vec2, asteroid_types: Vec<ItemId>) -> Self {
        Self {
            average_velocity,
            asteroids: HashMap::from_iter(
                asteroid_types.iter().map(|id| (*id, Default::default())),
            ),
            asteroid_respawns: HashMap::from_iter(
                asteroid_types.iter().map(|id| (*id, Default::default())),
            ),
            asteroid_types,
        }
    }

    #[inline]
    #[must_use]
    pub fn average_velocity(&self) -> &Vec2 {
        &self.average_velocity
    }

    #[inline]
    #[must_use]
    pub fn asteroid_types(&self) -> &Vec<ItemId> {
        &self.asteroid_types
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

use crate::components::{Asteroid, InSector};
use crate::constants;
use crate::persistence::{ComponentWithPersistentId, PersistentAsteroidId};
use crate::simulation::physics::ConstantVelocity;
use crate::simulation::prelude::SimulationTimestamp;
use crate::utils::{AsteroidEntityWithTimestamp, SectorEntity};
use bevy::math::Vec2;
use bevy::prelude::{Commands, Component};
use std::cmp::Ordering;
use std::collections::{BTreeSet, BinaryHeap};

/// A sector with this component features small asteroids floating through it.
#[derive(Component)]
pub struct SectorAsteroidComponent {
    pub average_velocity: Vec2,
    pub asteroids: BTreeSet<AsteroidEntityWithTimestamp>,
    pub asteroid_respawns: BinaryHeap<std::cmp::Reverse<RespawningAsteroidData>>,
}

impl SectorAsteroidComponent {
    pub fn new(average_velocity: Vec2) -> Self {
        Self {
            average_velocity,
            asteroids: BTreeSet::new(),
            asteroid_respawns: BinaryHeap::new(),
        }
    }

    /// How "Healthy" the asteroid field of this sector is... as in how many of its asteroids are currently spawned, in Range [0,1].
    pub fn remaining_percentage(&self) -> f32 {
        let spawned = self.asteroids.len();
        let despawned = self.asteroid_respawns.len();

        spawned as f32 / (spawned + despawned) as f32
    }

    /// Adds the given asteroid to this sector and inserts the [InSector] component to it.
    pub fn add_asteroid(
        &mut self,
        commands: &mut Commands,
        sector_entity: SectorEntity,
        entity: AsteroidEntityWithTimestamp,
    ) {
        self.asteroids.insert(entity);
        InSector::add_component(commands, sector_entity, entity.entity.into());
    }

    pub fn remove_asteroid_and_add_it_to_respawn_queue(
        &mut self,
        commands: &mut Commands,
        sector_entity: SectorEntity,
        entity: AsteroidEntityWithTimestamp,
    ) {
        self.asteroids.insert(entity);
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
            timestamp: value.despawn_timestamp + constants::ASTEROID_RESPAWN_TIME_MILLISECONDS,
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

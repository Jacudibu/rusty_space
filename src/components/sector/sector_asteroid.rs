use crate::components::InSector;
use crate::utils::{AsteroidEntityWithTimestamp, SectorEntity};
use bevy::math::Vec2;
use bevy::prelude::{Commands, Component};
use std::collections::{BTreeSet, BinaryHeap};

/// A sector with this component features small asteroids floating through it.
#[derive(Component)]
pub struct SectorAsteroidComponent {
    pub average_velocity: Vec2,
    pub asteroids: BTreeSet<AsteroidEntityWithTimestamp>,
    pub asteroid_respawns: BinaryHeap<std::cmp::Reverse<AsteroidEntityWithTimestamp>>,
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
        self.add_asteroid_in_place(entity);
        InSector::add_component(commands, sector_entity, entity.entity.into());
    }

    /// Adds asteroid to this sectors' asteroid set.
    pub fn add_asteroid_in_place(&mut self, entity: AsteroidEntityWithTimestamp) {
        self.asteroids.insert(entity);
    }
}

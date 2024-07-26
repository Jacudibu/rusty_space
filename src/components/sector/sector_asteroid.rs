use crate::components::Sector;
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

    /// Adds the given asteroid to this sector and inserts the [InSector] component to it.
    pub fn add_asteroid(
        &mut self,
        commands: &mut Commands,
        sector_entity: SectorEntity,
        entity: AsteroidEntityWithTimestamp,
    ) {
        self.add_asteroid_in_place(entity);
        Sector::in_sector(commands, sector_entity, entity.entity.into());
    }

    /// Adds asteroid to this sectors' asteroid set.
    pub fn add_asteroid_in_place(&mut self, entity: AsteroidEntityWithTimestamp) {
        self.asteroids.insert(entity);
    }
}

use crate::asteroids::SectorWasSpawnedEvent;
use crate::components::SectorAsteroidData;
use crate::utils::spawn_helpers::spawn_sector;
use crate::utils::SectorEntity;
use bevy::prelude::{Commands, EventWriter};
use hexx::{Hex, HexLayout};

pub struct SectorSpawnDataInstanceBuilder {
    pub coordinate: Hex,
    pub asteroids: Option<SectorAsteroidData>,
}

impl SectorSpawnDataInstanceBuilder {
    pub fn new(coordinate: Hex) -> Self {
        Self {
            coordinate,
            asteroids: None,
        }
    }

    pub fn with_asteroids(&mut self, asteroids: SectorAsteroidData) -> &mut Self {
        self.asteroids = Some(asteroids);
        self
    }

    pub fn build(
        &self,
        commands: &mut Commands,
        hex_layout: &HexLayout,
        spawn_events: &mut EventWriter<SectorWasSpawnedEvent>,
    ) -> SectorEntity {
        spawn_sector(
            commands,
            hex_layout,
            self.coordinate,
            self.asteroids,
            spawn_events,
        )
    }
}

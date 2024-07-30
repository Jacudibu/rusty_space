mod despawning;
mod fading;
mod helpers;
mod plugin;
mod respawning;
mod spawning;

use crate::utils::SectorEntity;
use bevy::prelude::Event;

pub use {
    despawning::AsteroidWasFullyMinedEvent, plugin::AsteroidPlugin, respawning::respawn_asteroids,
};

// TODO: Move this into some sector module instead of housing it here
#[derive(Event)]
pub struct SectorWasSpawnedEvent {
    pub(crate) sector: SectorEntity,
}

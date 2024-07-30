mod despawning;
mod fading;
pub(crate) mod helpers;
mod plugin;
mod respawning;

pub use {
    despawning::AsteroidWasFullyMinedEvent, plugin::AsteroidPlugin, respawning::respawn_asteroids,
};

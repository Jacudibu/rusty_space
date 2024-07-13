use crate::asteroids::despawning::AsteroidWasFullyMinedEvent;
use crate::asteroids::fading::{FadingAsteroidsIn, FadingAsteroidsOut};
use crate::asteroids::SectorWasSpawnedEvent;
use crate::asteroids::{despawning, fading, respawning, spawning};
use bevy::app::{App, Plugin, Update};
use bevy::prelude::{on_event, IntoSystemConfigs};

/// ### General Idea
/// Every Sector may have asteroids inside it, defined by its [SectorAsteroidData].
/// Every Sector keeps track of its "alive" Asteroids inside their `asteroids` variable.
/// There is a fixed amount of asteroids within each sector.
/// Once an asteroid is mined or floats outside the sector borders, it gets removed from the sectors
/// `asteroid` variable and its visibility is turned off.
/// Another system keeps track of an ordered Queue with all "dead" asteroids, resetting their position
/// and visibility once a set [SimulationTimestamp] has been reached.
pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FadingAsteroidsOut>()
            .init_resource::<FadingAsteroidsIn>()
            .add_event::<SectorWasSpawnedEvent>()
            .add_event::<AsteroidWasFullyMinedEvent>()
            .add_systems(
                Update,
                (
                    spawning::spawn_asteroids_for_new_sector
                        .run_if(on_event::<SectorWasSpawnedEvent>()),
                    despawning::make_asteroids_disappear_when_they_leave_sector
                        .before(spawning::spawn_asteroids_for_new_sector),
                    despawning::on_asteroid_was_fully_mined
                        .run_if(on_event::<AsteroidWasFullyMinedEvent>()),
                    respawning::respawn_asteroids,
                    fading::fade_asteroids_out,
                    fading::fade_asteroids_in,
                    //draw_asteroid_debug_gizmos,
                ),
            );
    }
}

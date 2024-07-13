use crate::asteroids::fading::FadingAsteroidsOut;
use crate::components::{Asteroid, InSector, Sector};
use crate::constants;
use crate::utils::{
    AsteroidEntity, AsteroidEntityWithTimestamp, SimulationTime, SimulationTimestamp,
};
use bevy::prelude::{Event, EventReader, Query, Res, ResMut};

#[derive(Event)]
pub struct AsteroidWasFullyMinedEvent {
    pub asteroid: AsteroidEntity,
    pub despawn_timer: SimulationTimestamp,
}

pub fn on_asteroid_was_fully_mined(
    mut events: EventReader<AsteroidWasFullyMinedEvent>,
    mut fading_asteroids: ResMut<FadingAsteroidsOut>,
    mut asteroids: Query<(&InSector, &mut Asteroid)>,
    mut sectors: Query<&mut Sector>,
) {
    for event in events.read() {
        let (asteroid_sector, mut asteroid) = asteroids.get_mut(event.asteroid.into()).unwrap();
        let mut sector = sectors.get_mut(asteroid_sector.sector.into()).unwrap();

        // I wish there was some way to do this without reconstructing this object
        let asteroid_entity = AsteroidEntityWithTimestamp {
            entity: event.asteroid,
            timestamp: event.despawn_timer,
        };

        // Asteroid might have already started despawning naturally, so test if it was still inside.
        if sector.asteroids.remove(&asteroid_entity) {
            despawn_asteroid(
                &mut fading_asteroids,
                asteroid_entity,
                &mut sector,
                &mut asteroid,
            );
        }
    }
}

pub fn despawn_asteroid(
    fading_asteroids: &mut ResMut<FadingAsteroidsOut>,
    mut asteroid_entity: AsteroidEntityWithTimestamp,
    sector: &mut Sector,
    asteroid: &mut Asteroid,
) {
    asteroid_entity
        .timestamp
        .add_milliseconds(constants::ASTEROID_RESPAWN_TIME_MILLISECONDS);
    asteroid
        .state
        .toggle_and_add_milliseconds(constants::ASTEROID_RESPAWN_TIME_MILLISECONDS);
    fading_asteroids.asteroids.insert(asteroid_entity.entity);
    sector
        .asteroid_respawns
        .push(std::cmp::Reverse(asteroid_entity));
}

/// Needs to run before [spawning::spawn_asteroids_for_new_sector] in order to ensure no new asteroids are spawned which aren't yet synced.
/// Technically this doesn't need to run every frame, given the super slow speed of asteroids.
pub fn make_asteroids_disappear_when_they_leave_sector(
    mut fading_asteroids: ResMut<FadingAsteroidsOut>,
    mut sector: Query<&mut Sector>,
    mut asteroids: Query<&mut Asteroid>,
    simulation_time: Res<SimulationTime>,
) {
    let now = simulation_time.now();

    for mut sector in sector.iter_mut() {
        while let Some(next) = sector.asteroids.first() {
            if now.has_not_passed(next.timestamp) {
                break;
            }

            let asteroid_entity = sector.asteroids.pop_first().unwrap();
            let mut asteroid = asteroids.get_mut(asteroid_entity.entity.into()).unwrap();
            despawn_asteroid(
                &mut fading_asteroids,
                asteroid_entity,
                &mut sector,
                &mut asteroid,
            );
        }
    }
}

use crate::map_layout::MapLayout;
use crate::simulation::asteroids::fading::FadingAsteroidsOut;
use crate::simulation::asteroids::respawning;
use crate::simulation::physics::ConstantVelocity;
use crate::simulation::prelude::{SimulationTime, SimulationTimestamp};
use crate::simulation::transform::SimulationTransform;
use crate::utils::{AsteroidEntity, AsteroidEntityWithTimestamp};
use bevy::prelude::{Event, EventReader, Query, Res, ResMut, Vec2};
use common::components::{Asteroid, InSector, RespawningAsteroidData, Sector, SectorWithAsteroids};

#[derive(Event)]
pub struct AsteroidWasFullyMinedEvent {
    pub asteroid: AsteroidEntity,
    pub despawn_timer: SimulationTimestamp,
}

pub fn on_asteroid_was_fully_mined(
    mut events: EventReader<AsteroidWasFullyMinedEvent>,
    mut fading_asteroids: ResMut<FadingAsteroidsOut>,
    asteroids: Query<(
        &InSector,
        &Asteroid,
        &ConstantVelocity,
        &SimulationTransform,
    )>,
    mut sectors_with_asteroids: Query<(&Sector, &mut SectorWithAsteroids)>,
    map_layout: Res<MapLayout>,
) {
    for event in events.read() {
        let Ok((asteroid_sector, asteroid, velocity, transform)) =
            asteroids.get(event.asteroid.into())
        else {
            // This event is surprisingly late to the party!
            continue;
        };

        let (sector, mut asteroid_component) = sectors_with_asteroids
            .get_mut(asteroid_sector.sector.into())
            .unwrap();

        // I wish there was some way to do this without reconstructing this object
        let asteroid_entity = AsteroidEntityWithTimestamp {
            entity: event.asteroid,
            timestamp: event.despawn_timer,
        };

        // Asteroid might have already started despawning naturally if it wasn't removed...
        if asteroid_component
            .asteroids
            .get_mut(&asteroid.ore_item_id)
            .unwrap()
            .remove(&asteroid_entity)
        {
            let local_respawn_position =
                respawning::calculate_local_asteroid_respawn_position_asteroid_was_mined(
                    map_layout.hex_edge_vertices,
                    transform.translation - sector.world_pos,
                    velocity.velocity(),
                );

            initiate_despawn_animation(
                &mut fading_asteroids,
                asteroid_entity,
                &mut asteroid_component,
                asteroid,
                velocity,
                local_respawn_position,
            );
        }
    }
}

/// Needs to run before [spawning::spawn_asteroids_for_new_sector] in order to ensure no new asteroids are spawned which aren't yet synced.
/// Technically this doesn't need to run every frame, given the super slow speed of asteroids.
pub fn make_asteroids_disappear_when_they_leave_sector(
    mut fading_asteroids: ResMut<FadingAsteroidsOut>,
    mut sector_asteroids: Query<(&Sector, &mut SectorWithAsteroids)>,
    asteroids: Query<(&Asteroid, &ConstantVelocity, &SimulationTransform)>,
    simulation_time: Res<SimulationTime>,
) {
    let now = simulation_time.now();

    for (sector, mut asteroid_component) in sector_asteroids.iter_mut() {
        for item_id in &asteroid_component.asteroid_types().clone() {
            while let Some(next) = asteroid_component.asteroids[item_id].first() {
                if now.has_not_passed(next.timestamp) {
                    break;
                }

                let asteroid_entity = asteroid_component
                    .asteroids
                    .get_mut(item_id)
                    .unwrap()
                    .pop_first()
                    .unwrap();

                let (asteroid, velocity, transform) =
                    asteroids.get(asteroid_entity.entity.into()).unwrap();

                let local_respawn_position =
                    respawning::calculate_local_asteroid_respawn_position_asteroid_left_sector(
                        transform.translation - sector.world_pos,
                    );

                initiate_despawn_animation(
                    &mut fading_asteroids,
                    asteroid_entity,
                    &mut asteroid_component,
                    asteroid,
                    velocity,
                    local_respawn_position,
                );
            }
        }
    }
}

fn initiate_despawn_animation(
    fading_asteroids: &mut ResMut<FadingAsteroidsOut>,
    asteroid_entity: AsteroidEntityWithTimestamp,
    feature: &mut SectorWithAsteroids,
    asteroid: &Asteroid,
    velocity: &ConstantVelocity,
    local_respawn_position: Vec2,
) {
    feature
        .asteroid_respawns
        .get_mut(&asteroid.ore_item_id)
        .unwrap()
        .push(std::cmp::Reverse(RespawningAsteroidData::new(
            asteroid,
            velocity,
            local_respawn_position,
        )));

    fading_asteroids.asteroids.insert(asteroid_entity.entity);
}

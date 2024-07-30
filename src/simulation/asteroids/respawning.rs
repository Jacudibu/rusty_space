use crate::components::{Sector, SectorAsteroidComponent};
use crate::map_layout::MapLayout;
use crate::persistence::AsteroidIdMap;
use crate::simulation::asteroids::fading::FadingAsteroidsIn;
use crate::simulation::asteroids::helpers;
use crate::simulation::prelude::SimulationTime;
use crate::utils::spawn_helpers;
use crate::SpriteHandles;
use bevy::math::Vec2;
use bevy::prelude::{Commands, Entity, Query, Res, ResMut};

pub fn respawn_asteroids(
    mut commands: Commands,
    mut asteroid_id_map: ResMut<AsteroidIdMap>,
    sprites: Res<SpriteHandles>,
    mut fading_asteroids: ResMut<FadingAsteroidsIn>,
    mut sectors_with_asteroids: Query<(Entity, &Sector, &mut SectorAsteroidComponent)>,
    simulation_time: Res<SimulationTime>,
    map_layout: Res<MapLayout>,
) {
    let now = simulation_time.now();

    for (sector_entity, sector, mut asteroid_component) in sectors_with_asteroids.iter_mut() {
        while let Some(next) = asteroid_component.asteroid_respawns.peek() {
            if now.has_not_passed(next.0.timestamp) {
                break;
            }

            let next = asteroid_component.asteroid_respawns.pop().unwrap().0;

            let millis_until_asteroid_leaves_again =
                helpers::calculate_milliseconds_until_asteroid_leaves_hexagon(
                    map_layout.hex_edge_vertices,
                    next.local_respawn_position,
                    next.velocity,
                );

            let asteroid_entity = spawn_helpers::spawn_asteroid(
                &mut commands,
                &mut asteroid_id_map,
                &sprites,
                "Asteroid".to_string(),
                next.local_respawn_position + sector.world_pos,
                &mut asteroid_component,
                sector_entity.into(),
                next.velocity,
                next.ore_max,
                next.ore_max,
                next.angular_velocity * std::f32::consts::PI * 1000.0,
                next.angular_velocity,
                next.timestamp + millis_until_asteroid_leaves_again,
                true,
            );

            fading_asteroids.asteroids.insert(asteroid_entity);
        }
    }
}

pub fn calculate_local_asteroid_respawn_position_asteroid_was_mined(
    local_hexagon_edges: [[Vec2; 2]; 6],
    local_current_position: Vec2,
    velocity: Vec2,
) -> Vec2 {
    let mut despawn_intersection = None;

    for edge in local_hexagon_edges.iter() {
        if let Some(intersection) = helpers::intersect_lines(
            local_current_position,
            local_current_position + velocity * helpers::VELOCITY_MULTIPLIER,
            edge[0],
            edge[1],
        ) {
            despawn_intersection = Some(intersection);
            break;
        }
    }

    if let Some(result) = despawn_intersection {
        Vec2::new(-result.x, -result.y)
    } else {
        // Super rare edge case when asteroid was mined right on sector edge
        calculate_local_asteroid_respawn_position_asteroid_left_sector(local_current_position)
    }
}

pub fn calculate_local_asteroid_respawn_position_asteroid_left_sector(
    local_current_position: Vec2,
) -> Vec2 {
    Vec2::new(-local_current_position.x, -local_current_position.y)
}

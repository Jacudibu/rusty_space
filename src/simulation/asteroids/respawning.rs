use crate::components::{Asteroid, Sector, SectorFeature};
use crate::map_layout::MapLayout;
use crate::simulation::asteroids::fading::FadingAsteroidsIn;
use crate::simulation::asteroids::helpers;
use crate::simulation::prelude::{ConstantVelocity, SimulationTime, SimulationTransform};
use bevy::math::Vec2;
use bevy::prelude::{Query, Res, ResMut, Visibility};

pub fn respawn_asteroids(
    mut fading_asteroids: ResMut<FadingAsteroidsIn>,
    mut sector: Query<&mut Sector>,
    mut asteroid_query: Query<(
        &mut Asteroid,
        &mut SimulationTransform,
        &mut Visibility,
        &ConstantVelocity,
    )>,
    simulation_time: Res<SimulationTime>,
    map_layout: Res<MapLayout>,
) {
    let now = simulation_time.now();

    for mut sector in sector.iter_mut() {
        let sector_world_pos = sector.world_pos;
        let SectorFeature::Asteroids(feature) = &mut sector.feature else {
            continue;
        };

        while let Some(next) = feature.asteroid_respawns.peek() {
            if now.has_not_passed(next.0.timestamp) {
                break;
            }

            let mut asteroid_entity = feature.asteroid_respawns.pop().unwrap().0;

            let (mut asteroid, mut transform, mut visibility, velocity) = asteroid_query
                .get_mut(asteroid_entity.entity.into())
                .unwrap();

            let local_respawn_position = calculate_asteroid_respawn_position(
                map_layout.hex_edge_vertices,
                transform.translation - sector_world_pos,
                velocity.velocity,
            );

            let extra_millis = helpers::calculate_milliseconds_until_asteroid_leaves_hexagon(
                map_layout.hex_edge_vertices,
                local_respawn_position,
                velocity.velocity,
            );
            *visibility = Visibility::Inherited;
            transform
                .set_translation_and_skip_interpolation(local_respawn_position + sector_world_pos);
            asteroid_entity.timestamp.add_milliseconds(extra_millis);
            asteroid.state.toggle_and_add_milliseconds(extra_millis);
            asteroid.reset(&mut transform);
            fading_asteroids.asteroids.insert(asteroid_entity.entity);
            feature.asteroids.insert(asteroid_entity);
        }
    }
}

fn calculate_asteroid_respawn_position(
    local_hexagon_edges: [[Vec2; 2]; 6],
    local_current_position: Vec2,
    velocity: Vec2,
) -> Vec2 {
    // Avoid using randomness, so we don't need to sync anything over the network
    let mut best_distance = f32::MAX;
    let mut despawn_intersection = None;

    // TODO: Alternatively: Just store the mirrored despawn position and properly despawn the entity
    //   Overall, spawning and despawning should still be cheaper than keeping the movement simulation up,
    //   especially if the asteroid is mined very early in its lifecycle

    for edge in local_hexagon_edges.iter() {
        if let Some(intersection) = helpers::intersect_lines(
            local_current_position,
            local_current_position - velocity * helpers::VELOCITY_MULTIPLIER,
            edge[0],
            edge[1],
        ) {
            let distance = intersection.distance_squared(local_current_position);
            if distance < best_distance {
                best_distance = distance;
                despawn_intersection = Some(intersection);
            }
        }
    }

    let result =
        despawn_intersection.expect("Asteroids should always intersect with their hexagon!");
    Vec2::new(-result.x, -result.y)
}

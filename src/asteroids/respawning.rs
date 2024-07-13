use crate::asteroids::fading::FadingAsteroidsIn;
use crate::asteroids::helpers;
use crate::components::{Asteroid, Sector};
use crate::constants;
use crate::map_layout::MapLayout;
use crate::physics::ConstantVelocity;
use crate::utils::SimulationTime;
use bevy::math::Vec2;
use bevy::prelude::{Query, Res, ResMut, Transform, Visibility};

pub fn respawn_asteroids(
    mut fading_asteroids: ResMut<FadingAsteroidsIn>,
    mut sector: Query<&mut Sector>,
    mut asteroid_query: Query<(
        &mut Asteroid,
        &mut Transform,
        &mut Visibility,
        &ConstantVelocity,
    )>,
    simulation_time: Res<SimulationTime>,
    map_layout: Res<MapLayout>,
) {
    let now = simulation_time.now();

    for mut sector in sector.iter_mut() {
        while let Some(next) = sector.asteroid_respawns.peek() {
            if now.has_not_passed(next.0.timestamp) {
                break;
            }

            let mut asteroid_entity = sector.asteroid_respawns.pop().unwrap().0;

            let (mut asteroid, mut transform, mut visibility, velocity) = asteroid_query
                .get_mut(asteroid_entity.entity.into())
                .unwrap();

            let velocity = velocity.velocity.truncate();

            let local_respawn_position = calculate_asteroid_respawn_position(
                map_layout.hex_edge_vertices,
                transform.translation.truncate() - sector.world_pos,
                velocity,
            );

            let extra_millis = helpers::calculate_milliseconds_until_asteroid_leaves_hexagon(
                map_layout.hex_edge_vertices,
                local_respawn_position,
                velocity,
            );
            *visibility = Visibility::Inherited;
            transform.translation =
                (local_respawn_position + sector.world_pos).extend(constants::ASTEROID_LAYER);
            asteroid_entity.timestamp.add_milliseconds(extra_millis);
            asteroid.state.toggle_and_add_milliseconds(extra_millis);
            asteroid.reset(&mut transform);
            fading_asteroids.asteroids.insert(asteroid_entity.entity);
            sector.asteroids.insert(asteroid_entity);
        }
    }
}

fn calculate_asteroid_respawn_position(
    local_hexagon_edges: [[Vec2; 2]; 6],
    local_current_position: Vec2,
    velocity: Vec2,
) -> Vec2 {
    // Avoid using randomness, so we don't need to sync anything over the network
    let mut best_distance = 0.0;
    let mut best_intersection = None;

    // TODO: Alternatively: Store the despawn position, then mirror it to the opposing hexagon side.

    for edge in local_hexagon_edges.iter() {
        if let Some(intersection) = helpers::intersect_lines(
            local_current_position,
            local_current_position - velocity * helpers::VELOCITY_MULTIPLIER,
            edge[0],
            edge[1],
        ) {
            let distance = intersection.distance_squared(local_current_position);
            if distance > best_distance {
                best_distance = distance;
                best_intersection = Some(intersection);
            }
        }
    }

    best_intersection.expect("Asteroids should always intersect with their hexagon!")
}

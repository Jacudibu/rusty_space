use crate::simulation::prelude::Milliseconds;
use crate::utils::intersections;
use bevy::math::Vec2;

/// A big number to ensure the velocity vector is long enough to actually intersect with hexagon boundaries
pub const VELOCITY_MULTIPLIER: f32 = 100000000.0;

/// ## Panics
/// If the point is not within the hexagon or velocity is 0.
pub fn calculate_milliseconds_until_asteroid_leaves_hexagon(
    local_hexagon_edges: [[Vec2; 2]; 6],
    local_spawn_position: Vec2,
    velocity: Vec2,
) -> Milliseconds {
    debug_assert!(velocity.length_squared() > 0.0);
    let mut time = -1.0;

    for edge in local_hexagon_edges.iter() {
        if let Some(intersection) = intersections::intersect_lines(
            local_spawn_position,
            local_spawn_position + velocity * VELOCITY_MULTIPLIER,
            edge[0],
            edge[1],
        ) {
            let distance = intersection.distance(local_spawn_position);
            if distance < 1.0 {
                // Too close, might happen when we are right on the edge
                continue;
            }

            time = distance / velocity.length();
            break;
        }
    }

    (time * 1000.0) as Milliseconds - 1 // Fade duration is ~1, so might as well subtract that for extra fancyness
}

use crate::utils::Milliseconds;
use bevy::math::Vec2;

/// Intersects the two lines `(a1, a2)` and `(b1, b2)` and returns the point of intersection.
pub fn intersect_lines(a1: Vec2, a2: Vec2, b1: Vec2, b2: Vec2) -> Option<Vec2> {
    let denominator = (b2.y - b1.y) * (a2.x - a1.x) - (b2.x - b1.x) * (a2.y - a1.y);

    if denominator.abs() < f32::EPSILON {
        return None; // Lines are parallel
    }

    let ua = ((b2.x - b1.x) * (a1.y - b1.y) - (b2.y - b1.y) * (a1.x - b1.x)) / denominator;
    let ub = ((a2.x - a1.x) * (a1.y - b1.y) - (a2.y - a1.y) * (a1.x - b1.x)) / denominator;

    if (0.0..=1.0).contains(&ua) && (0.0..=1.0).contains(&ub) {
        let x = a1.x + ua * (a2.x - a1.x);
        let y = a1.y + ua * (a2.y - a1.y);
        return Some(Vec2 { x, y });
    }

    None
}

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
        if let Some(intersection) = intersect_lines(
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

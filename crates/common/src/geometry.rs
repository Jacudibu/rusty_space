use bevy::prelude::{Vec2, Vec3};

/// Tests whether a rectangle overlaps with a circle. The rectangle *must* be axis-aligned.
pub fn overlap_rectangle_with_circle_axis_aligned(
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
    circle_center: Vec3,
    circle_radius: f32,
) -> bool {
    let closest_x = circle_center.x.max(left).min(right);
    let closest_y = circle_center.y.max(bottom).min(top);

    let distance_x_squared = (circle_center.x - closest_x).powi(2);
    let distance_y_squared = (circle_center.y - closest_y).powi(2);

    distance_x_squared + distance_y_squared <= circle_radius * circle_radius
}

/// Tests whether two circles overlap.
pub fn overlap_circle_with_circle(
    circle_a_center: Vec3,
    circle_a_radius: f32,
    circle_b_center: Vec3,
    circle_b_radius: f32,
) -> bool {
    let x = circle_a_center.x - circle_b_center.x;
    let y = circle_a_center.y - circle_b_center.y;
    let distance_squared = x * x + y * y;
    distance_squared <= (circle_a_radius + circle_b_radius).powi(2)
}

#[allow(dead_code)]
pub fn overlap_point_with_hexagon(point: Vec3, hexagon_edges: [[Vec2; 2]; 6]) -> bool {
    let mut intersections = 0;
    for [a, b] in hexagon_edges {
        let is_between_y = (a.y > point.y) != (b.y > point.y);
        if is_between_y && (point.x < (b.x - a.x) * (point.y - a.y) / (b.y - a.y) + a.x) {
            intersections += 1;
        }
    }

    intersections == 1
}

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

/// Tests whether circle intersects a line defined by two points.
/// https://mathworld.wolfram.com/Circle-LineIntersection.html
pub fn intersect_line_with_circle(
    a: Vec2,
    b: Vec2,
    circle_center: Vec2,
    circle_radius: f32,
) -> bool {
    // Move everything so the circle is at (0,0)
    let ax = a.x - circle_center.x;
    let ay = a.y - circle_center.y;
    let bx = b.x - circle_center.x;
    let by = b.y - circle_center.y;

    let dx = bx - ax;
    let dy = by - ay;
    let dr = (dx * dx + dy * dy).sqrt();

    let d = ax * by - ay * bx;

    0.0 < (circle_radius * circle_radius * dr * dr) - (d * d)
}

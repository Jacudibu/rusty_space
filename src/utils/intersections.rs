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

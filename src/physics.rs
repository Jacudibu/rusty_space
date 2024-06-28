use crate::components::Velocity;
use bevy::prelude::{Query, Res, Time, Transform, Vec3};

pub fn move_things(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    query.par_iter_mut().for_each(|(mut transform, velocity)| {
        transform.rotate_z(velocity.angular * time.delta_seconds());

        let forward = transform.up();
        transform.translation += forward * velocity.forward * time.delta_seconds();
    });
}

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

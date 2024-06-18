use crate::components::Velocity;
use bevy::prelude::{Query, Res, Time, Transform};

pub fn move_things(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    query.par_iter_mut().for_each(|(mut transform, velocity)| {
        transform.rotate_z(velocity.angular * time.delta_seconds());

        let forward = transform.up();
        transform.translation += forward * velocity.forward * time.delta_seconds();
    });
}

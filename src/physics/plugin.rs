use crate::components::{InSector, Velocity};
use bevy::app::PostUpdate;
use bevy::prelude::{App, Plugin, Query, Res, Time, Transform, With};

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, move_things);
    }
}

fn move_things(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity), With<InSector>>) {
    query.par_iter_mut().for_each(|(mut transform, velocity)| {
        transform.rotate_z(velocity.angular * time.delta_seconds());

        let forward = transform.up();
        transform.translation += forward * velocity.forward * time.delta_seconds();
    });
}

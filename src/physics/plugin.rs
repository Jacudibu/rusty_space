use crate::components::InSector;
use crate::physics::constant_velocity::ConstantVelocity;
use crate::physics::ShipVelocity;
use bevy::app::PostUpdate;
use bevy::prelude::{App, Plugin, Query, Res, Time, Transform, With};

/// Beautifully simplified fake physics.
pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, (move_ships, move_constant_stuff));
    }
}

fn move_ships(time: Res<Time>, mut ships: Query<(&mut Transform, &ShipVelocity), With<InSector>>) {
    ships.par_iter_mut().for_each(|(mut transform, velocity)| {
        transform.rotate_z(velocity.angular * time.delta_seconds());

        let forward = transform.up();
        transform.translation += forward * velocity.forward * time.delta_seconds();
    });
}

fn move_constant_stuff(time: Res<Time>, mut items: Query<(&mut Transform, &ConstantVelocity)>) {
    items.par_iter_mut().for_each(|(mut transform, velocity)| {
        transform.translation += velocity.velocity * time.delta_seconds();
    });
}

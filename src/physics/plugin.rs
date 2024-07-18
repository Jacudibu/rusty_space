use crate::components::InSector;
use crate::physics::constant_velocity::ConstantVelocity;
use crate::physics::ShipVelocity;
use crate::simulation_transform::SimulationTransform;
use bevy::prelude::{App, FixedPostUpdate, Plugin, Query, Res, Time, With};

/// Beautifully simplified fake physics.
pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedPostUpdate, (move_ships, move_constant_stuff));
    }
}

fn move_ships(
    time: Res<Time>,
    mut ships: Query<(&mut SimulationTransform, &ShipVelocity), With<InSector>>,
) {
    ships.par_iter_mut().for_each(|(mut transform, velocity)| {
        transform.rotate(velocity.angular * time.delta_seconds());

        let forward = transform.forward();
        transform.translation += forward * velocity.forward * time.delta_seconds();
    });
}

fn move_constant_stuff(
    time: Res<Time>,
    mut items: Query<(&mut SimulationTransform, &ConstantVelocity)>,
) {
    items.par_iter_mut().for_each(|(mut transform, velocity)| {
        transform.rotate(velocity.sprite_rotation * time.delta_seconds());
        transform.translation += velocity.velocity * time.delta_seconds();
    });
}

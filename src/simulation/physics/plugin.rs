use crate::components::{InSector, IsDocked};
use crate::simulation::physics::ShipVelocity;
use crate::simulation::physics::constant_velocity::ConstantVelocity;
use crate::simulation::physics::orbit_system::orbit_system;
use crate::simulation::transform::simulation_transform::SimulationTransform;
use crate::states::SimulationState;
use bevy::prelude::{
    App, FixedPostUpdate, IntoScheduleConfigs, Plugin, Query, Res, Time, With, Without, in_state,
};

/// Beautifully simplified fake physics.
pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedPostUpdate,
            (move_ships, move_constant_stuff, orbit_system)
                .run_if(in_state(SimulationState::Running)),
        );
    }
}

fn move_ships(
    time: Res<Time>,
    mut ships: Query<
        (&mut SimulationTransform, &ShipVelocity),
        (With<InSector>, Without<IsDocked>),
    >,
) {
    ships.par_iter_mut().for_each(|(mut transform, velocity)| {
        transform.rotate(velocity.angular * time.delta_secs());

        let forward = transform.forward();
        transform.translation += forward * velocity.forward * time.delta_secs();
    });
}

fn move_constant_stuff(
    time: Res<Time>,
    mut items: Query<(&mut SimulationTransform, &ConstantVelocity)>,
) {
    items.par_iter_mut().for_each(|(mut transform, velocity)| {
        transform.rotate(velocity.sprite_rotation() * time.delta_secs());
        transform.translation += velocity.velocity() * time.delta_secs();
    });
}

use crate::physics::orbit_system::orbit_system;
use bevy::prelude::{
    App, FixedPostUpdate, IntoScheduleConfigs, Plugin, Query, Res, Time, With, Without, in_state,
};
use common::components::constant_velocity::ConstantVelocity;
use common::components::ship_velocity::ShipVelocity;
use common::components::{InSector, IsDocked};
use common::simulation_transform::SimulationTransform;
use common::states::SimulationState;

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

use crate::simulation::prelude::SimulationTime;
use crate::simulation::transform::simulation_transform::SimulationTransform;
use crate::states::SimulationState;
use bevy::app::{App, FixedPreUpdate, Plugin, Update};
use bevy::prelude::{
    in_state, DetectChanges, Fixed, IntoSystemConfigs, Local, Mut, Query, Res, Time, Transform,
    ViewVisibility,
};

/// Interpolates the transforms used for the visual representation to their respective simulation values.
pub struct SimulationTransformPlugin;
impl Plugin for SimulationTransformPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedPreUpdate,
            copy_old_transform_values.run_if(in_state(SimulationState::Running)),
        );
        app.add_systems(
            Update,
            interpolate_transforms.run_if(in_state(SimulationState::Running)),
        );
    }
}

fn copy_old_transform_values(mut transforms: Query<Mut<SimulationTransform>>) {
    transforms.par_iter_mut().for_each(|mut x| {
        let did_change = x.is_changed();
        x.copy_old_values(did_change);
    });
}

fn interpolate_transforms(
    time: Res<Time<Fixed>>,
    simulation_time: Res<SimulationTime>,
    mut all_ships: Query<(&SimulationTransform, &mut Transform, &ViewVisibility)>,
    mut update_all_next_frame: Local<bool>,
) {
    let update_all = *update_all_next_frame && !simulation_time.is_changed();
    if simulation_time.is_changed() {
        *update_all_next_frame = true;
    }

    let overstep_fraction = time.overstep_fraction();

    if update_all {
        all_ships
            .par_iter_mut()
            .for_each(|(simulation_transform, mut transform, _)| {
                simulation_transform.update_transform(&mut transform, overstep_fraction);
            });
    } else {
        all_ships
            .par_iter_mut()
            .for_each(|(simulation_transform, mut transform, visibility)| {
                if !visibility.get() {
                    return;
                }

                simulation_transform.update_transform(&mut transform, overstep_fraction);
            });
    }
}

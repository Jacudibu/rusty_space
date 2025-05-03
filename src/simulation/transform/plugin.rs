use crate::simulation::prelude::SimulationTime;
use crate::simulation::prelude::simulation_transform::SimulationScale;
use crate::simulation::transform::simulation_transform::SimulationTransform;
use crate::states::SimulationState;
use bevy::app::{App, FixedPreUpdate, Plugin, Update};
use bevy::prelude::{
    DetectChanges, Fixed, IntoScheduleConfigs, Local, Mut, Query, Res, Time, Transform,
    ViewVisibility, in_state,
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

fn copy_old_transform_values(
    mut transforms: Query<(Mut<SimulationTransform>, Mut<SimulationScale>)>,
) {
    transforms
        .par_iter_mut()
        .for_each(|(mut transform, mut scale)| {
            if transform.is_changed() {
                transform.copy_old_values();
            }

            if scale.is_changed() {
                scale.copy_old_values();
            }
        });
}

fn interpolate_transforms(
    time: Res<Time<Fixed>>,
    simulation_time: Res<SimulationTime>,
    mut all_transforms: Query<(
        &SimulationTransform,
        &SimulationScale,
        &mut Transform,
        &ViewVisibility,
    )>,
    mut update_all_next_frame: Local<bool>,
) {
    let update_all = *update_all_next_frame && !simulation_time.is_changed();
    if simulation_time.is_changed() {
        *update_all_next_frame = true;
    }

    let overstep_fraction = time.overstep_fraction();

    if update_all {
        all_transforms.par_iter_mut().for_each(
            |(simulation_transform, simulation_scale, mut transform, _)| {
                simulation_transform.update_transform(&mut transform, overstep_fraction);
                simulation_scale.update_transform(&mut transform, overstep_fraction);
            },
        );
    } else {
        all_transforms.par_iter_mut().for_each(
            |(simulation_transform, simulation_scale, mut transform, visibility)| {
                if !visibility.get() {
                    return;
                }

                simulation_transform.update_transform(&mut transform, overstep_fraction);
                simulation_scale.update_transform(&mut transform, overstep_fraction);
            },
        );
    }
}

use crate::simulation_time::SimulationTime;
use crate::states::SimulationState;
use bevy::prelude::{App, FixedFirst, IntoScheduleConfigs, Plugin, Res, ResMut, Time, in_state};
use bevy::time::Fixed;

/// Sets up tracking for [SimulationTime]
pub(crate) struct SimulationTimePlugin;
impl Plugin for SimulationTimePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SimulationTime::default());
        app.add_systems(
            FixedFirst,
            update
                .after(bevy::time::TimeSystem)
                .run_if(in_state(SimulationState::Running)),
        );
    }
}

/// Should always run **after** [bevy::time::TimeSystem]
fn update(mut simulation_time: ResMut<SimulationTime>, bevy_time: Res<Time<Fixed>>) {
    simulation_time.advance(bevy_time.delta());
}

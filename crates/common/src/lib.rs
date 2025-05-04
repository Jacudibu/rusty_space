use crate::simulation_time::SimulationTimePlugin;
use bevy::app::{App, Plugin};
use bevy::prelude::AppExtStates;

pub mod constants;
pub mod simulation_time;
pub mod states;

/// Registers all the things inside the common crate.
pub struct CommonPlugin;
impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<states::ApplicationState>();
        app.add_sub_state::<states::SimulationState>();
        app.add_sub_state::<states::MenuState>();

        app.add_plugins(SimulationTimePlugin);
    }
}

use crate::simulation_time::SimulationTimePlugin;
use crate::simulation_transform::plugin::SimulationTransformPlugin;
use bevy::app::{App, Plugin};
use bevy::prelude::AppExtStates;

pub mod components;
pub mod constants;
pub mod events;
pub mod game_data;
pub mod geometry;
pub mod interpolation;
pub mod session_data;
pub mod simulation_time;
pub mod simulation_transform;
pub mod states;
pub mod system_sets;
pub mod types;

/// Registers all the things inside the common crate.
pub struct CommonPlugin;
impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<states::ApplicationState>();
        app.add_sub_state::<states::SimulationState>();
        app.add_sub_state::<states::MenuState>();

        app.add_plugins(SimulationTimePlugin);
        app.add_plugins(SimulationTransformPlugin);
    }
}

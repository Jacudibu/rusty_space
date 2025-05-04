use crate::simulation_time::SimulationTimePlugin;
use bevy::app::{App, Plugin};
use bevy::prelude::AppExtStates;

pub mod components;
pub mod constants;
pub mod enums;
pub mod events;
pub mod game_data;
pub mod persistent_entity_id;
pub mod price_range;
pub mod session_data;
pub mod simulation_time;
pub mod states;
pub mod types;

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

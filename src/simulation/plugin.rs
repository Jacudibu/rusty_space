use crate::constants;
use crate::simulation::{asteroids, physics, production, ship_ai, time, transform};
use crate::states::{ApplicationState, SimulationState};
use bevy::prelude::{
    in_state, App, ButtonInput, IntoSystemConfigs, KeyCode, NextState, Plugin, Res, ResMut, State,
    Time, Update, Virtual,
};
use bevy::time::Fixed;

pub struct SimulationPlugin;
impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Time::<Fixed>::from_hz(constants::TICKS_PER_SECOND));
        app.add_plugins((
            asteroids::AsteroidPlugin,
            physics::PhysicsPlugin,
            production::ProductionPlugin,
            ship_ai::ShipAiPlugin,
            time::SimulationTimePlugin,
            transform::SimulationTransformPlugin,
        ));
        app.add_systems(
            Update,
            toggle_pause.run_if(in_state(ApplicationState::InGame)),
        );
    }
}

pub fn toggle_pause(
    input: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<SimulationState>>,
    mut next_state: ResMut<NextState<SimulationState>>,
    mut time: ResMut<Time<Virtual>>,
) {
    if input.just_pressed(KeyCode::Space) {
        next_state.set(match current_state.get() {
            SimulationState::Running => {
                time.pause();
                SimulationState::Paused
            }
            SimulationState::Paused => {
                time.unpause();
                SimulationState::Running
            }
        });
    }
}

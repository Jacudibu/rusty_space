use crate::simulation::precomputed_orbit_directions::PrecomputedOrbitDirections;
use crate::simulation::*;
use bevy::prelude::{
    App, ButtonInput, IntoScheduleConfigs, KeyCode, NextState, Plugin, Res, ResMut, State, Time,
    Update, Virtual, in_state,
};
use bevy::time::Fixed;
use common::constants;
use common::states::{ApplicationState, SimulationState};

pub struct SimulationPlugin;
impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PrecomputedOrbitDirections>();
        app.insert_resource(Time::<Fixed>::from_hz(constants::TICKS_PER_SECOND));
        app.add_plugins((
            asteroids::AsteroidPlugin,
            construction_site_updater::ConstructionSiteUpdaterPlugin,
            physics::PhysicsPlugin,
            production::ProductionPlugin,
            ship_ai::ShipAiPlugin,
        ));
        app.add_systems(
            Update,
            toggle_pause.run_if(in_state(ApplicationState::InGame)),
        );
        app.add_systems(
            Update, // TODO: Depending on our orbit velocity, this should be running in FixedUpdate or even less often and use SimulationTransform
            moving_gate_connections::update_gate_connections
                .run_if(in_state(SimulationState::Running)),
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

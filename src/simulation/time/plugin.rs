use crate::simulation::prelude::SimulationTime;
use bevy::prelude::{App, FixedFirst, IntoSystemConfigs, Plugin, Res, ResMut, Time};
use bevy::time::Fixed;

pub struct SimulationTimePlugin;
impl Plugin for SimulationTimePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SimulationTime::default());
        app.add_systems(FixedFirst, update.after(bevy::time::TimeSystem));
    }
}

/// Should always run **after** [bevy::time::TimeSystem]
fn update(mut simulation_time: ResMut<SimulationTime>, bevy_time: Res<Time<Fixed>>) {
    simulation_time.advance(bevy_time.delta());
}

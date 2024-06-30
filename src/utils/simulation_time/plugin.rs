use crate::utils::SimulationTime;
use bevy::prelude::{App, First, IntoSystemConfigs, Plugin, Res, ResMut, Time, Virtual};

pub struct SimulationTimePlugin;
impl Plugin for SimulationTimePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SimulationTime::default());
        app.add_systems(First, update.after(bevy::time::TimeSystem));
    }
}

/// Should always run **after** [bevy::time::TimeSystem]
fn update(mut simulation_time: ResMut<SimulationTime>, bevy_time: Res<Time<Virtual>>) {
    simulation_time.advance(bevy_time.delta());
}

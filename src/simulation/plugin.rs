use crate::constants;
use crate::simulation::{asteroids, physics, production, ship_ai, time, transform};
use bevy::prelude::{App, Plugin, Time};
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
    }
}

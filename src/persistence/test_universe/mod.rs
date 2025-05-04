use crate::initialize_data;
use bevy::app::{App, Plugin};
use bevy::prelude::{IntoScheduleConfigs, Startup, World};
use common::game_data::AsteroidManifest;
use common::session_data::SessionData;

mod coordinates;
mod gate_test_data;
mod sector_test_data;
mod ship_test_data;
mod station_test_data;

pub struct TestUniverseDataPlugin;
impl Plugin for TestUniverseDataPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_test_universe.after(initialize_data));
    }
}

pub fn load_test_universe(world: &mut World) {
    SessionData::initialize_mock_data(world);

    world.insert_resource(sector_test_data::create_test_data(
        world
            .get_resource::<AsteroidManifest>()
            .expect("Manifests should be parsed before TestUniversePlugin is added!"),
    ));
    world.insert_resource(gate_test_data::create_test_data());
    world.insert_resource(station_test_data::create_test_data());
    world.insert_resource(ship_test_data::create_test_data());
}

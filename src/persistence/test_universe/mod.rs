use crate::game_data::AsteroidManifest;
use crate::session_data::SessionData;
use bevy::app::{App, Plugin};

mod coordinates;
mod gate_test_data;
mod sector_test_data;
mod ship_test_data;
mod station_test_data;

pub struct TestUniverseDataPlugin;
impl Plugin for TestUniverseDataPlugin {
    fn build(&self, app: &mut App) {
        SessionData::initialize_mock_data(app.world_mut());

        app.insert_resource(sector_test_data::create_test_data(
            app.world()
                .get_resource::<AsteroidManifest>()
                .expect("Manifests should be parsed before TestUniversePlugin is added!"),
        ));
        app.insert_resource(gate_test_data::create_test_data());
        app.insert_resource(station_test_data::create_test_data());
        app.insert_resource(ship_test_data::create_test_data());
    }
}

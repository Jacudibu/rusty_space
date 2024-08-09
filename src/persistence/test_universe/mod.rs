use bevy::app::{App, Plugin};
mod coordinates;
mod gate_test_data;
mod sector_test_data;
mod ship_test_data;
mod station_test_data;

pub struct TestUniverseDataPlugin;
impl Plugin for TestUniverseDataPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(sector_test_data::create_test_data());
        app.insert_resource(gate_test_data::create_test_data());
        app.insert_resource(station_test_data::create_test_data());
        app.insert_resource(ship_test_data::create_test_data());
    }
}

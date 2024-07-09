use bevy::app::{App, Plugin};
mod coordinates;
mod gates;
mod sectors;
mod ships;
mod stations;

pub struct TestUniversePlugin;
impl Plugin for TestUniversePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(sectors::create_test_data());
        app.insert_resource(gates::create_test_data());
        app.insert_resource(stations::create_test_data());
        app.insert_resource(ships::create_test_data());
    }
}

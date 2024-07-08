use bevy::app::{App, Plugin};

mod gates;
mod sectors;

pub struct TestUniversePlugin;
impl Plugin for TestUniversePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(sectors::create_test_data());
        app.insert_resource(gates::create_test_data());
    }
}

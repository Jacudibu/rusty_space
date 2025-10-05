use bevy::app::{App, Plugin};
use bevy::ecs::system::SystemParam;
use bevy::prelude::{Res, World};

pub mod ship_configs;

pub use ship_configs::{
    ShipConfigId, ShipConfiguration, ShipConfigurationAddedEvent, ShipConfigurationManifest,
    ShipConfigurationVersions,
};

/// Data that's dynamically created and indexed whilst playing and needs to be persisted in between sessions.
#[derive(SystemParam)]
pub struct SessionData<'w> {
    pub ship_configurations: Res<'w, ShipConfigurationManifest>,
}

impl SessionData<'_> {
    pub fn initialize_mock_data(world: &mut World) {
        let ship_configs = ShipConfigurationManifest::mock_data(world);
        world.insert_resource(ship_configs);
    }
}

pub struct SessionDataPlugin;
impl Plugin for SessionDataPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ShipConfigurationAddedEvent>();
    }
}

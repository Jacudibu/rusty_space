use bevy::ecs::system::SystemParam;
use bevy::prelude::{Res, World};

mod ship_configs;

pub use ship_configs::{
    ShipConfigId, ShipConfiguration, ShipConfigurationManifest, ShipConfigurationVersions,
    DEBUG_SHIP_CONFIG,
};

/// Data that's dynamically created and indexed whilst playing and needs to be persisted in between sessions.
#[derive(SystemParam)]
pub struct SessionData<'w> {
    pub ship_configurations: Res<'w, ShipConfigurationManifest>,
}

impl<'w> SessionData<'w> {
    pub fn initialize_mock_data(world: &mut World) {
        let ship_configs = ShipConfigurationManifest::mock_data(world);
        world.insert_resource(ship_configs);
    }
}

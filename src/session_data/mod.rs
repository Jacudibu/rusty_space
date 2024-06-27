use bevy::prelude::Resource;
use bevy::utils::HashMap;

mod ship_configuration;

pub use ship_configuration::*;

/// Data that's dynamically created and indexed whilst playing and needs to be persisted in between sessions.
#[derive(Resource, Default)]
pub struct SessionData {
    pub ship_configurations: HashMap<ShipConfigId, ShipConfiguration>,
}

impl SessionData {
    pub fn mock_data() -> Self {
        Self {
            ship_configurations: HashMap::from([(
                DEBUG_SHIP_CONFIG,
                ShipConfiguration {
                    id: DEBUG_SHIP_CONFIG,
                },
            )]),
        }
    }
}

mod ship_configuration;
mod ship_configuration_manifest;
mod ship_configuration_versions;
mod version;
mod versioned_id;

use crate::create_id_constants;

use crate::session_data::ship_configs::versioned_id::VersionedId;
#[allow(unused)]
pub use {
    ship_configuration::EngineStats, ship_configuration::EngineTuning,
    ship_configuration::ShipConfiguration,
    ship_configuration_manifest::ShipConfigurationAddedEvent,
    ship_configuration_manifest::ShipConfigurationManifest,
    ship_configuration_versions::ShipConfigurationVersions,
};

pub type ShipConfigId = VersionedId<ShipConfigurationVersions>;

create_id_constants!(
    ShipConfigId,
    MOCK_TRANSPORT_SHIP_CONFIG,
    MOCK_MINING_SHIP_CONFIG,
    MOCK_HARVESTING_SHIP_CONFIG,
    MOCK_CONSTRUCTION_SHIP_CONFIG
);

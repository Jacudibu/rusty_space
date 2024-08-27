mod ship_configuration;
mod ship_configuration_manifest;
mod ship_configuration_versions;
mod version;
mod versioned_id;

use crate::session_data::ship_configs::versioned_id::VersionedId;
pub use {
    ship_configuration::EngineStats, ship_configuration::EngineTuning,
    ship_configuration::ShipConfiguration, ship_configuration_manifest::ShipConfigurationManifest,
    ship_configuration_versions::ShipConfigurationVersions,
};

pub type ShipConfigId = VersionedId<ShipConfigurationVersions>;

const MOCK_TRANSPORT_SHIP_CONFIG_NAME: &str = "transport";
const MOCK_MINING_SHIP_CONFIG_NAME: &str = "miner";
const MOCK_HARVESTING_SHIP_CONFIG_NAME: &str = "harvester";

pub const MOCK_TRANSPORT_SHIP_CONFIG: ShipConfigId =
    ShipConfigId::from_name(MOCK_TRANSPORT_SHIP_CONFIG_NAME);
pub const MOCK_MINING_SHIP_CONFIG: ShipConfigId =
    ShipConfigId::from_name(MOCK_MINING_SHIP_CONFIG_NAME);
pub const MOCK_HARVESTING_SHIP_CONFIG: ShipConfigId =
    ShipConfigId::from_name(MOCK_HARVESTING_SHIP_CONFIG_NAME);

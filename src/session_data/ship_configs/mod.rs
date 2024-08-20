mod ship_configuration;
mod ship_configuration_manifest;
mod ship_configuration_versions;
mod version;
mod versioned_id;

use crate::session_data::ship_configs::versioned_id::VersionedId;
pub use {
    ship_configuration::ShipConfiguration, ship_configuration_manifest::ShipConfigurationManifest,
    ship_configuration_versions::ShipConfigurationVersions,
};

pub type ShipConfigId = VersionedId<ShipConfigurationVersions>;

const DEBUG_SHIP_CONFIG_NAME: &str = "config_a";
pub const DEBUG_SHIP_CONFIG: ShipConfigId = ShipConfigId::from_name(DEBUG_SHIP_CONFIG_NAME);

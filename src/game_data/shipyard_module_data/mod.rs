mod shipyard_module;
mod shipyard_module_manifest;

use leafwing_manifest::identifier::Id;

pub use {shipyard_module::ShipyardModuleData, shipyard_module_manifest::ShipyardModuleManifest};

pub type ShipyardModuleId = Id<ShipyardModuleData>;

pub const MOCK_SHIPYARD_MODULE_ID: ShipyardModuleId = ShipyardModuleId::from_name("shipyard_a");

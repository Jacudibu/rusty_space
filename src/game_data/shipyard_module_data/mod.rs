mod shipyard_module_definition;
mod shipyard_module_manifest;

use leafwing_manifest::identifier::Id;

pub use {
    shipyard_module_definition::ShipyardModuleDefinition,
    shipyard_module_manifest::ShipyardModuleManifest,
};

pub type ShipyardModuleId = Id<ShipyardModuleDefinition>;

pub const MOCK_SHIPYARD_MODULE_ID: ShipyardModuleId = ShipyardModuleId::from_name("shipyard_a");

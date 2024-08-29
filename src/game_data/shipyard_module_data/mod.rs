mod shipyard_module;
mod shipyard_module_manifest;

use crate::create_id_constants;
use leafwing_manifest::identifier::Id;

pub use {shipyard_module::ShipyardModuleData, shipyard_module_manifest::ShipyardModuleManifest};

pub type ShipyardModuleId = Id<ShipyardModuleData>;

create_id_constants!(ShipyardModuleId, MOCK_SHIPYARD_MODULE);

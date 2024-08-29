mod production_module;
mod production_module_manifest;

use leafwing_manifest::identifier::Id;

use crate::create_id_constants;
pub use {production_module::*, production_module_manifest::*};

pub type ProductionModuleId = Id<ProductionModuleData>;

create_id_constants!(
    ProductionModuleId,
    MOCK_PRODUCTION_MODULE_A,
    MOCK_PRODUCTION_MODULE_B,
    MOCK_PRODUCTION_MODULE_C
);

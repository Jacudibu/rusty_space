mod production_module;
mod production_module_manifest;

use leafwing_manifest::identifier::Id;

use crate::create_id_constants;
pub use {production_module::*, production_module_manifest::*};

pub type ProductionModuleId = Id<ProductionModuleData>;

create_id_constants!(
    ProductionModuleId,
    SILICA_PRODUCTION_MODULE,
    REFINED_METALS_PRODUCTION_MODULE,
    WAFERS_PRODUCTION_MODULE
);

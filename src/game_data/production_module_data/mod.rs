mod production_module;
mod production_module_manifest;

use leafwing_manifest::identifier::Id;

pub use {production_module::*, production_module_manifest::*};

pub type ProductionModuleId = Id<ProductionModuleData>;

pub const MOCK_PRODUCTION_MODULE_A_ID: ProductionModuleId = ProductionModuleId::from_name("prod_a");
pub const MOCK_PRODUCTION_MODULE_B_ID: ProductionModuleId = ProductionModuleId::from_name("prod_b");
pub const MOCK_PRODUCTION_MODULE_C_ID: ProductionModuleId = ProductionModuleId::from_name("prod_c");

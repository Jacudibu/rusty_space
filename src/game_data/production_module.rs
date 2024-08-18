use crate::game_data::RecipeId;
use leafwing_manifest::identifier::Id;

pub type ProductionModuleId = Id<ProductionModuleDefinition>;

pub const PRODUCTION_MODULE_A_ID: ProductionModuleId = ProductionModuleId::from_name("prod_a");
pub const PRODUCTION_MODULE_B_ID: ProductionModuleId = ProductionModuleId::from_name("prod_b");
pub const PRODUCTION_MODULE_C_ID: ProductionModuleId = ProductionModuleId::from_name("prod_c");

/// Defines the costs and capabilities of a single Production Line
pub struct ProductionModuleDefinition {
    /// Unique ID to differentiate between recipes
    pub id: ProductionModuleId,
    /// User Facing name thingy
    pub name: String,
    /// List of things that can be produced
    pub available_recipes: Vec<RecipeId>,
}

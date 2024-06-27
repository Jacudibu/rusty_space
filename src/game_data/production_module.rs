use crate::game_data::RecipeId;

pub type ProductionModuleId = u32;

pub const PRODUCTION_MODULE_A_ID: ProductionModuleId = 1;
pub const PRODUCTION_MODULE_B_ID: ProductionModuleId = 2;
pub const PRODUCTION_MODULE_C_ID: ProductionModuleId = 3;

/// Defines the costs and capabilities of a single Production Line
pub struct ProductionModuleDefinition {
    /// Unique ID to differentiate between recipes
    pub id: ProductionModuleId,
    /// User Facing name thingy
    pub name: String,
    /// List of things that can be produced
    pub available_recipes: Vec<RecipeId>,
}

use crate::game_data::production_module_data::ProductionModuleId;
use crate::game_data::RecipeId;
use serde::Deserialize;

/// Defines the costs and capabilities of a single Production Line
#[derive(Deserialize)]
pub struct ProductionModuleDefinition {
    /// Unique ID to differentiate between recipes
    pub id: ProductionModuleId,
    /// User Facing name thingy
    pub name: String,
    /// List of things that can be produced
    pub available_recipes: Vec<RecipeId>,
}

use crate::game_data::production_data::ProductionModuleId;
use crate::game_data::RecipeId;
use serde::Deserialize;

/// Defines the costs and capabilities of a single Production Line
#[derive(Deserialize)]
pub struct ProductionModule {
    /// Unique ID to differentiate between recipes
    pub id: ProductionModuleId,
    /// User Facing name thingy
    pub name: String,
    /// List of things that can be produced
    pub available_recipes: Vec<RecipeId>,
}

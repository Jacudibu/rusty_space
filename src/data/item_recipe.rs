use crate::data::ItemId;
use crate::simulation_time::SimulationSeconds;

pub type RecipeId = u32;

pub struct ItemRecipe {
    /// Unique ID to differentiate between recipes
    pub id: RecipeId,
    /// Useful to differentiate if an item has multiple recipes
    pub name: String,
    /// How long it will take to process this recipe once, in seconds
    pub duration: SimulationSeconds,
    /// The required ingredients to get production starting
    pub input: Vec<ItemRecipeElement>,
    /// Yields of a single production run
    pub output: Vec<ItemRecipeElement>,
}

pub struct ItemRecipeElement {
    pub item_id: ItemId,
    pub amount: u32,
}

use crate::game_data::{ProductionModuleId, RecipeId};
use crate::utils::SimulationTimestamp;
use bevy::prelude::Component;
use bevy::utils::HashMap;

#[derive(Component)]
pub struct ProductionComponent {
    pub modules: HashMap<ProductionModuleId, ProductionModule>,
}

pub struct ProductionModule {
    pub amount: u32,
    pub recipe: RecipeId,
    pub current_run_finished_at: Option<SimulationTimestamp>,
}

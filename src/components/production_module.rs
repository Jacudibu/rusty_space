use crate::data::RecipeId;
use bevy::prelude::Component;

#[derive(Component)]
pub struct ProductionModule {
    pub recipe: Option<RecipeId>,
    pub current_run_finished_at: Option<u32>,
}

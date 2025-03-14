use crate::game_data::{ProductionModuleId, RecipeId};
use crate::simulation::prelude::SimulationTimestamp;
use bevy::prelude::Component;
use bevy::utils::HashMap;

/// Component used for item production/processing facilities.
#[derive(Component)]
pub struct ProductionComponent {
    /// The modules which have been built inside this component, each managing their own production queues.
    pub modules: HashMap<ProductionModuleId, ProductionModule>,
}

/// Represents all production modules with a specific [ProductionModuleId] inside a [ProductionComponent].
pub struct ProductionModule {
    /// The amount of modules available
    pub amount: u32,

    /// A list of recipes which have been queued up
    /// [ProductionQueueElement] which are set to repeat will be immediately queued up again once they get popped.
    pub queued_recipes: Vec<ProductionQueueElement>,

    /// A list of recipes which are currently being processed.
    // TODO: With bigger station sizes, using some sort of priority queue would be more efficient here
    pub running_recipes: Vec<RunningProductionQueueElement>,
}

/// Represents a single element inside a production queue.
pub struct ProductionQueueElement {
    /// The [RecipeId] of the queued production run.
    pub recipe: RecipeId,

    /// Whether this queue element should be added back into its queue once it has been started.
    pub is_repeating: bool,
}

/// Represents a single running production... run.
pub struct RunningProductionQueueElement {
    /// The [RecipeId] of the queued production run.
    pub recipe: RecipeId,

    /// A [SimulationTimestamp] for when this run is finished.
    pub finished_at: SimulationTimestamp,
}

use crate::game_data::{ProductionModuleId, RecipeId};
use crate::simulation::prelude::SimulationTimestamp;
use bevy::prelude::Component;
use bevy::utils::HashMap;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

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

    /// A priority queue containing the recipes which are currently being processed.
    pub running_recipes: BinaryHeap<RunningProductionQueueElement>,
}

impl Default for ProductionModule {
    fn default() -> Self {
        Self {
            amount: 1,
            queued_recipes: Default::default(),
            running_recipes: Default::default(),
        }
    }
}

/// Represents a single element inside a production queue.
pub struct ProductionQueueElement {
    /// The [RecipeId] of the queued production run.
    pub recipe: RecipeId,

    /// Whether this queue element should be added back into its queue once it has been started.
    pub is_repeating: bool,
}

/// Represents a single running production... run.
#[derive(Eq, PartialEq)]
pub struct RunningProductionQueueElement {
    /// The [RecipeId] of the queued production run.
    pub recipe: RecipeId,

    /// A [SimulationTimestamp] for when this run is finished.
    pub finished_at: SimulationTimestamp,
}

impl PartialOrd<Self> for RunningProductionQueueElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RunningProductionQueueElement {
    fn cmp(&self, other: &Self) -> Ordering {
        // Inverted ordering so heap.max is our min element
        other.finished_at.cmp(&self.finished_at)
    }
}

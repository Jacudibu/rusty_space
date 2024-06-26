use crate::production::production_started_event::ProductionStartedEvent;
use crate::simulation_time::SimulationSeconds;
use bevy::prelude::{Entity, Resource};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

/// Keeps track of all ongoing production runs within in the ECS.
///
/// By using a binary heap to store references and timers to all ongoing production,
/// testing for finished production runs is O(1), and starting a new run is O(1)~ + O(log n).
#[derive(Resource)]
pub struct GlobalProductionState {
    elements: BinaryHeap<SingleProductionState>,
}

impl Default for GlobalProductionState {
    fn default() -> Self {
        Self {
            elements: BinaryHeap::with_capacity(200),
        }
    }
}

impl GlobalProductionState {
    pub fn insert(&mut self, value: SingleProductionState) {
        self.elements.push(value);
    }
    pub fn peek(&self) -> Option<&SingleProductionState> {
        self.elements.peek()
    }
    pub fn pop(&mut self) -> Option<SingleProductionState> {
        self.elements.pop()
    }
}

// TODO: we need to keep track of the thing that's produced, otherwise Eq might be wrong as soon as we produce more than one item
#[derive(Eq, PartialEq)]
pub struct SingleProductionState {
    pub entity: Entity,
    pub finished_at: SimulationSeconds,
}

impl From<&ProductionStartedEvent> for SingleProductionState {
    fn from(value: &ProductionStartedEvent) -> Self {
        SingleProductionState {
            entity: value.entity,
            finished_at: value.finishes_at,
        }
    }
}

impl Ord for SingleProductionState {
    fn cmp(&self, other: &Self) -> Ordering {
        // Inverted ordering so heap.max is our min element
        other.finished_at.cmp(&self.finished_at)
    }
}

impl PartialOrd for SingleProductionState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Inverted ordering so heap.max is our min element
        Some(other.finished_at.cmp(&self.finished_at))
    }
}

use crate::production::production_kind::ProductionKind;
use crate::production::state::{GlobalProductionState, SingleProductionState};
use crate::utils::Milliseconds;
use bevy::prelude::{Entity, Event, EventReader, ResMut};

#[derive(Event)]
pub struct ProductionStartedEvent {
    pub entity: Entity,
    pub kind: ProductionKind,
    pub finishes_at: Milliseconds,
}

impl ProductionStartedEvent {
    pub fn new(entity: Entity, kind: ProductionKind, finishes_at: Milliseconds) -> Self {
        Self {
            entity,
            kind,
            finishes_at,
        }
    }
}

pub fn on_production_started(
    mut global_production_state: ResMut<GlobalProductionState>,
    mut productions: EventReader<ProductionStartedEvent>,
) {
    for event in productions.read() {
        global_production_state.insert(SingleProductionState::from(event))
    }
}

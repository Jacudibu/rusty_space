use crate::production::production_kind::ProductionKind;
use crate::production::state::{GlobalProductionState, SingleProductionState};
use bevy::prelude::{Entity, Message, MessageReader, ResMut};
use common::simulation_time::SimulationTimestamp;

#[derive(Message)]
pub struct ProductionStartedEvent {
    pub entity: Entity,
    pub kind: ProductionKind,
    pub finishes_at: SimulationTimestamp,
}

impl ProductionStartedEvent {
    pub fn new(entity: Entity, kind: ProductionKind, finishes_at: SimulationTimestamp) -> Self {
        Self {
            entity,
            kind,
            finishes_at,
        }
    }
}

pub fn on_production_started(
    mut global_production_state: ResMut<GlobalProductionState>,
    mut productions: MessageReader<ProductionStartedEvent>,
) {
    for event in productions.read() {
        global_production_state.insert(SingleProductionState::from(event))
    }
}

use crate::data::ProductionModuleId;
use crate::production::state::{GlobalProductionState, SingleProductionState};
use crate::simulation_time::SimulationSeconds;
use bevy::prelude::{Entity, Event, EventReader, ResMut};

#[derive(Event)]
pub struct ProductionStartedEvent {
    pub entity: Entity,
    pub module_id: ProductionModuleId,
    pub finishes_at: SimulationSeconds,
}

impl ProductionStartedEvent {
    pub fn new(
        entity: Entity,
        module_id: ProductionModuleId,
        finishes_at: SimulationSeconds,
    ) -> Self {
        Self {
            entity,
            module_id,
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

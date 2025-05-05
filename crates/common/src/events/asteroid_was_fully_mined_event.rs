use crate::simulation_time::SimulationTimestamp;
use crate::types::entity_wrappers::AsteroidEntity;
use bevy::prelude::Event;

#[derive(Event)]
pub struct AsteroidWasFullyMinedEvent {
    pub asteroid: AsteroidEntity,
    pub despawn_timer: SimulationTimestamp,
}

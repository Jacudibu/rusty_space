use crate::simulation_time::SimulationTimestamp;
use crate::types::entity_wrappers::AsteroidEntity;
use bevy::prelude::Message;

#[derive(Message)]
pub struct AsteroidWasFullyMinedEvent {
    pub asteroid: AsteroidEntity,
    pub despawn_timer: SimulationTimestamp,
}

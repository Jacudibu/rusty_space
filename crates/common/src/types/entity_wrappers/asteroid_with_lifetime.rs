use crate::simulation_time::SimulationTimestamp;
use crate::types::entity_wrappers::AsteroidEntity;
use bevy::prelude::Entity;
use std::cmp::Ordering;

#[derive(Copy, Clone)]
pub struct AsteroidEntityWithTimestamp {
    pub entity: AsteroidEntity,
    pub timestamp: SimulationTimestamp,
}

impl Eq for AsteroidEntityWithTimestamp {}

impl From<AsteroidEntityWithTimestamp> for Entity {
    fn from(value: AsteroidEntityWithTimestamp) -> Self {
        value.entity.into()
    }
}
impl From<&AsteroidEntityWithTimestamp> for Entity {
    fn from(value: &AsteroidEntityWithTimestamp) -> Self {
        value.entity.into()
    }
}

impl PartialEq<Self> for AsteroidEntityWithTimestamp {
    fn eq(&self, other: &Self) -> bool {
        other.entity == self.entity
    }
}

impl PartialOrd<Self> for AsteroidEntityWithTimestamp {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AsteroidEntityWithTimestamp {
    fn cmp(&self, other: &Self) -> Ordering {
        self.timestamp
            .cmp(&other.timestamp)
            .then_with(|| self.entity.cmp(&other.entity))
    }
}

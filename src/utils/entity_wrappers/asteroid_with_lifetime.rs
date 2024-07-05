use crate::utils::{AsteroidEntity, SimulationTimestamp};
use std::cmp::Ordering;

#[derive(Copy, Clone)]
pub struct AsteroidEntityWithTimestamp {
    pub entity: AsteroidEntity,
    pub timestamp: SimulationTimestamp,
}

impl Eq for AsteroidEntityWithTimestamp {}

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
        // Inverted ordering so heap.max is our min element
        other.timestamp.cmp(&self.timestamp)
    }
}

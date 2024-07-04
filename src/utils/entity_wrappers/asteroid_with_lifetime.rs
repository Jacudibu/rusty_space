use crate::utils::{AsteroidEntity, SimulationTimestamp};
use std::cmp::Ordering;

#[derive(Copy, Clone)]
pub struct AsteroidEntityWithLifetime {
    pub entity: AsteroidEntity,
    pub despawn_at: SimulationTimestamp,
}

impl Eq for AsteroidEntityWithLifetime {}

impl PartialEq<Self> for AsteroidEntityWithLifetime {
    fn eq(&self, other: &Self) -> bool {
        other.entity == self.entity
    }
}

impl PartialOrd<Self> for AsteroidEntityWithLifetime {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AsteroidEntityWithLifetime {
    fn cmp(&self, other: &Self) -> Ordering {
        // Inverted ordering so heap.max is our min element
        other.despawn_at.cmp(&self.despawn_at)
    }
}

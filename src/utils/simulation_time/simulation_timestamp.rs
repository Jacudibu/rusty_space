use crate::utils::simulation_time::MILLIS_PER_SECOND;
use crate::utils::Milliseconds;
use std::cmp::Ordering;
use std::time::Duration;

/// Represents the current Timestamp in Milliseconds since session start.
#[derive(Copy, Clone)]
pub struct CurrentSimulationTimestamp(Milliseconds);
impl CurrentSimulationTimestamp {
    #[inline]
    pub(in crate::utils::simulation_time) fn from(milliseconds: Milliseconds) -> Self {
        Self(milliseconds)
    }

    /// Returns the current simulation timestamp in [Milliseconds].
    pub fn get(&self) -> Milliseconds {
        self.0
    }

    /// Returns true if the provided timestamp lies within the past.
    #[inline]
    pub fn has_passed(&self, timestamp: SimulationTimestamp) -> bool {
        self.0 >= timestamp.0
    }

    /// Returns true if the provided timestamp lies within the future.
    #[inline]
    pub fn has_not_passed(&self, timestamp: SimulationTimestamp) -> bool {
        self.0 < timestamp.0
    }

    /// Returns a new [SimulationTimestamp] with the specified amount of seconds added to it.
    #[inline]
    pub fn add_seconds(&self, seconds: u64) -> SimulationTimestamp {
        SimulationTimestamp(self.0 + seconds * MILLIS_PER_SECOND)
    }

    /// Returns a new [SimulationTimestamp] with the specified amount of milliseconds added to it.
    #[inline]
    pub fn add_milliseconds(&self, milliseconds: u64) -> SimulationTimestamp {
        SimulationTimestamp(self.0 + milliseconds)
    }

    /// Returns the [Duration] required to reach the given timestamp.
    pub fn remaining_time(&self, timestamp: SimulationTimestamp) -> Duration {
        if self.has_passed(timestamp) {
            Duration::ZERO
        } else {
            Duration::from_millis(timestamp.0 - self.0)
        }
    }
}

/// Represents a specific Timestamp in Milliseconds since session start.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SimulationTimestamp(Milliseconds);

impl SimulationTimestamp {
    pub const MIN: SimulationTimestamp = SimulationTimestamp(Milliseconds::MIN);
    pub const MAX: SimulationTimestamp = SimulationTimestamp(Milliseconds::MAX);

    pub fn milliseconds(&self) -> Milliseconds {
        self.0
    }

    /// Adds a set amount of Milliseconds to this timestamp.
    pub fn add_milliseconds(&mut self, amount: Milliseconds) {
        self.0 += amount;
    }
}

impl From<CurrentSimulationTimestamp> for SimulationTimestamp {
    fn from(value: CurrentSimulationTimestamp) -> Self {
        Self(value.0)
    }
}

impl Ord for SimulationTimestamp {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for SimulationTimestamp {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.0.cmp(&other.0))
    }
}

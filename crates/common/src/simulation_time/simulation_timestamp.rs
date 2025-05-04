use crate::simulation_time::{MILLIS_PER_SECOND, Milliseconds};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::ops::Add;
use std::time::Duration;

/// Represents the current Timestamp in Milliseconds since session start.
#[derive(Copy, Clone)]
pub struct CurrentSimulationTimestamp(Milliseconds);
impl CurrentSimulationTimestamp {
    #[inline]
    pub fn from(milliseconds: Milliseconds) -> Self {
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

#[cfg(test)]
impl From<Milliseconds> for CurrentSimulationTimestamp {
    fn from(value: Milliseconds) -> Self {
        Self(value)
    }
}

/// Represents a specific Timestamp in Milliseconds since session start.
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct SimulationTimestamp(Milliseconds);

impl SimulationTimestamp {
    pub const MIN: SimulationTimestamp = SimulationTimestamp(Milliseconds::MIN);
    pub const MAX: SimulationTimestamp = SimulationTimestamp(Milliseconds::MAX);

    #[inline]
    pub fn milliseconds(&self) -> Milliseconds {
        self.0
    }

    /// Adds a set amount of Milliseconds to this timestamp.
    #[inline]
    pub fn add_milliseconds(&mut self, amount: Milliseconds) {
        self.0 += amount;
    }

    /// Returns true if the provided timestamp lies within the past when compared to Self.
    #[inline]
    pub fn has_passed(&self, other: &Self) -> bool {
        self.0 >= other.0
    }

    /// Returns true if the provided timestamp lies within the future when compared to Self.
    #[inline]
    pub fn has_not_passed(&self, other: &Self) -> bool {
        self.0 < other.0
    }
}

impl From<CurrentSimulationTimestamp> for SimulationTimestamp {
    fn from(value: CurrentSimulationTimestamp) -> Self {
        Self(value.0)
    }
}
impl From<&CurrentSimulationTimestamp> for SimulationTimestamp {
    fn from(value: &CurrentSimulationTimestamp) -> Self {
        Self(value.0)
    }
}

impl From<Milliseconds> for SimulationTimestamp {
    /// Can be used in universe creation to spread out initial idle checks across multiple frames
    fn from(value: Milliseconds) -> Self {
        Self(value)
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

impl Add<Milliseconds> for SimulationTimestamp {
    type Output = Self;

    fn add(self, rhs: Milliseconds) -> Self::Output {
        Self(self.0 + rhs)
    }
}

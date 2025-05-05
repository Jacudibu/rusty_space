use crate::simulation_time::SimulationTimestamp;
use crate::types::ship_behaviors::ShipBehaviorData;
use bevy::prelude::Component;
use std::ops::{Deref, DerefMut};

/// A ShipTask can be attached to ship entities in order to have them do stuff.
#[derive(Component)]
pub struct ShipBehavior<T: ShipBehaviorData> {
    /// The [SimulationTimestamp] at which we search for a new task.
    pub next_idle_update: SimulationTimestamp,

    /// The data for our behavior.
    data: T,
}

impl<T: ShipBehaviorData> ShipBehavior<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            // Forces an update during the next tick.
            next_idle_update: SimulationTimestamp::MIN,
        }
    }
}

impl<T: ShipBehaviorData> Deref for ShipBehavior<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T: ShipBehaviorData> DerefMut for ShipBehavior<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

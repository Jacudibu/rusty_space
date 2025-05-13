use bevy::prelude::Component;
use common::types::ship_tasks::ShipTaskData;
use std::ops::{Deref, DerefMut};

/// A ShipTask can be attached to ship entities in order to have them do stuff.
#[derive(Component, Debug)]
pub struct ShipTask<T: ShipTaskData> {
    data: T,
}

impl<T: ShipTaskData> ShipTask<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }
}

impl<T: ShipTaskData> Deref for ShipTask<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T: ShipTaskData> DerefMut for ShipTask<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

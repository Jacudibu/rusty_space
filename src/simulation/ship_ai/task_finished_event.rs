use crate::simulation::prelude::TaskComponent;
use bevy::prelude::{Entity, Event};
use std::marker::PhantomData;

#[derive(Event, Copy, Clone)]
pub struct TaskFinishedEvent<T: TaskComponent> {
    t: PhantomData<T>,
    pub entity: Entity,
}

impl<T: TaskComponent> TaskFinishedEvent<T> {
    pub fn new(entity: Entity) -> Self {
        Self {
            entity,
            t: PhantomData,
        }
    }
}

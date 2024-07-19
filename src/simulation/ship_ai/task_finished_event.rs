use bevy::prelude::{Component, Entity, Event};
use std::marker::PhantomData;

#[derive(Event, Copy, Clone)]
pub struct TaskFinishedEvent<T: Component> {
    t: PhantomData<T>,
    pub entity: Entity,
}

impl<T: Component> TaskFinishedEvent<T> {
    pub fn new(entity: Entity) -> Self {
        Self {
            entity,
            t: PhantomData,
        }
    }
}

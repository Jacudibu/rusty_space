use crate::simulation::ship_ai::tasks::{Construct, ExchangeWares, Undock, UseGate};
use bevy::ecs::system::SystemParam;
use bevy::prelude::{Component, Entity, Event, EventWriter};
use std::marker::PhantomData;

#[derive(Event, Copy, Clone)]
pub struct TaskStartedEvent<T: Component> {
    t: PhantomData<T>,
    pub entity: Entity,
}

impl<T: Component> TaskStartedEvent<T> {
    pub fn new(entity: Entity) -> Self {
        Self {
            entity,
            t: PhantomData,
        }
    }
}

#[derive(SystemParam)]
pub struct AllTaskStartedEventWriters<'w> {
    pub exchange_wares: EventWriter<'w, TaskStartedEvent<ExchangeWares>>,
    pub use_gate: EventWriter<'w, TaskStartedEvent<UseGate>>,
    pub undock: EventWriter<'w, TaskStartedEvent<Undock>>,
    pub construct: EventWriter<'w, TaskStartedEvent<Construct>>,
}

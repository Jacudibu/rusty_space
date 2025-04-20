use crate::simulation::prelude::{ConstructTaskComponent, TaskComponent};
use crate::simulation::ship_ai::tasks::{ExchangeWares, Undock, UseGate};
use crate::utils::ShipEntity;
use bevy::ecs::system::SystemParam;
use bevy::prelude::{Entity, Event, EventWriter};
use std::marker::PhantomData;

#[derive(Event, Copy, Clone)]
pub struct TaskCompletedEvent<T: TaskComponent> {
    t: PhantomData<T>,
    pub entity: Entity,
}

impl<T: TaskComponent> TaskCompletedEvent<T> {
    pub fn new(entity: Entity) -> Self {
        Self {
            entity,
            t: PhantomData,
        }
    }
}

#[derive(Event, Copy, Clone)]
pub struct TaskCancelledEvent<T: TaskComponent> {
    t: PhantomData<T>,
    pub entity: Entity,
}

impl<T: TaskComponent> TaskCancelledEvent<T> {
    pub fn new(entity: Entity) -> Self {
        Self {
            entity,
            t: PhantomData,
        }
    }
}

#[derive(Event, Copy, Clone)]
pub struct TaskStartedEvent<T: TaskComponent> {
    t: PhantomData<T>,
    pub entity: ShipEntity,
}

impl<T: TaskComponent> TaskStartedEvent<T> {
    pub fn new(entity: ShipEntity) -> Self {
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
    pub construct: EventWriter<'w, TaskStartedEvent<ConstructTaskComponent>>,
}

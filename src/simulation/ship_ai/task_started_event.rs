use crate::simulation::prelude::TaskComponent;
use crate::simulation::ship_ai::tasks::{ConstructTaskComponent, ExchangeWares, Undock, UseGate};
use crate::utils::ShipEntity;
use bevy::ecs::system::SystemParam;
use bevy::prelude::{Event, EventWriter};
use std::marker::PhantomData;

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

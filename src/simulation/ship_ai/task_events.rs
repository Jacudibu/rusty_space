use crate::simulation::prelude::{ConstructTaskComponent, TaskComponent};
use crate::simulation::ship_ai::tasks::{ExchangeWares, Undock, UseGate};
use crate::utils::ShipEntity;
use bevy::ecs::system::SystemParam;
use bevy::prelude::{Event, EventWriter};
use std::marker::PhantomData;

pub mod event_kind {
    pub struct Started;
    pub struct Completed;
    pub struct Canceled;
}

/// Indicates that a Task was just started.
pub type TaskStartedEvent<T> = TaskEvent<T, event_kind::Started>;
/// Indicates that a Task was just completed.
pub type TaskCompletedEvent<T> = TaskEvent<T, event_kind::Completed>;
/// Indicates that a Task was just canceled.
pub type TaskCanceledEvent<T> = TaskEvent<T, event_kind::Canceled>;

/// Generic base class for all task-related events.
/// Use [TaskStartedEvent], [TaskCompletedEvent] and [TaskCanceledEvent] for better readability.
#[derive(Event, Copy, Clone)]
pub struct TaskEvent<Task: TaskComponent, Kind> {
    /// The [TaskComponent] type a
    task: PhantomData<Task>,
    /// See [event_kind] for the various kinds of TaskEvents we got.
    kind: PhantomData<Kind>,
    /// The entity connected to this task.
    pub entity: ShipEntity,
}

impl<Task: TaskComponent, Kind> TaskEvent<Task, Kind> {
    pub fn new(entity: ShipEntity) -> Self {
        Self {
            entity,
            task: PhantomData,
            kind: PhantomData,
        }
    }
}

/// A [SystemParam] collection of all [TaskStartedEvent] EventWriters.
/// Right now this needs to be passed into all behaviors to initiate new tasks.
#[derive(SystemParam)]
pub struct AllTaskStartedEventWriters<'w> {
    pub exchange_wares: EventWriter<'w, TaskStartedEvent<ExchangeWares>>,
    pub use_gate: EventWriter<'w, TaskStartedEvent<UseGate>>,
    pub undock: EventWriter<'w, TaskStartedEvent<Undock>>,
    pub construct: EventWriter<'w, TaskStartedEvent<ConstructTaskComponent>>,
}

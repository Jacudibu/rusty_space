use crate::types::entity_wrappers::ShipEntity;
use crate::types::ship_tasks::{
    AwaitingSignal, Construct, DockAtEntity, ExchangeWares, HarvestGas, MineAsteroid, MoveToEntity,
    MoveToPosition, RequestAccess, ShipTaskData, Undock, UseGate,
};
use bevy::ecs::system::SystemParam;
use bevy::prelude::{Entity, Event, EventWriter};
use std::marker::PhantomData;

pub mod event_kind {
    pub struct Started;
    pub struct Completed;
    pub struct Aborted;
    pub struct Canceled;
}

/// Indicates that a new active Task was just started.
pub type TaskStartedEvent<T> = TaskEvent<T, event_kind::Started>;
/// Indicates that an active Task was just completed.
pub type TaskCompletedEvent<T> = TaskEvent<T, event_kind::Completed>;

/// Indicates that an active Task was aborted during execution.
/// This usually happens due to entity destruction or user interaction.
pub type TaskAbortedEvent<T> = TaskEventWithData<T, event_kind::Aborted>;

/// Indicates that a Task inside the TaskQueue was canceled.
/// This usually happens due to entity destruction or user interaction.
pub type TaskCanceledEvent<T> = TaskEventWithData<T, event_kind::Canceled>;

/// Generic base class for all task-related events which don't require to contain a copy of the task data.
/// Use [TaskStartedEvent], [TaskCompletedEvent] for better readability.
#[derive(Event, Copy, Clone)]
pub struct TaskEvent<Task: ShipTaskData, Kind> {
    /// The type of [ShipTask].
    task_data: PhantomData<Task>,
    /// See [event_kind] for the various kinds of TaskEvents we got.
    kind: PhantomData<Kind>,
    /// The entity connected to this task.
    pub entity: ShipEntity,
}

impl<Task: ShipTaskData, Kind> TaskEvent<Task, Kind> {
    pub fn new(entity: ShipEntity) -> Self {
        Self {
            entity,
            task_data: PhantomData,
            kind: PhantomData,
        }
    }
}

/// Generic base class for all task-related events which require task data.
/// Use [TaskAbortedEvent] and [TaskCanceledEvent] for better readability.
#[derive(Event, Copy, Clone)]
pub struct TaskEventWithData<TaskData: ShipTaskData, Kind> {
    /// See [event_kind] for the various kinds of TaskEvents we got.
    kind: PhantomData<Kind>,
    /// The entity connected to this task.
    pub entity: ShipEntity,
    /// The type of [ShipTask].
    pub task_data: TaskData,
}

impl<TaskData: ShipTaskData, Kind> TaskEventWithData<TaskData, Kind> {
    pub fn new(entity: ShipEntity, task_data: TaskData) -> Self {
        Self {
            entity,
            task_data,
            kind: PhantomData,
        }
    }
}

/// Event to add a series of new tasks into a [TaskQueue] in order to execute the provided task.
/// Adding the target task is enough!
///
/// e.g. adding [ExchangeWares] automatically populates the task queue with tasks to move and dock at the target.
#[derive(Event)]
pub struct InsertTaskIntoQueueCommand<Task: ShipTaskData> {
    /// The entity which should receive the task
    pub entity: Entity,
    /// The task data which should be inserted into the queue.
    pub task_data: Task,
}

/// A [SystemParam] collection of all [TaskStartedEvent] EventWriters.
/// Right now this needs to be passed into all behaviors to initiate new tasks.
#[derive(SystemParam)]
pub struct AllTaskStartedEventWriters<'w> {
    pub exchange_wares: EventWriter<'w, TaskStartedEvent<ExchangeWares>>,
    pub use_gate: EventWriter<'w, TaskStartedEvent<UseGate>>,
    pub undock: EventWriter<'w, TaskStartedEvent<Undock>>,
    pub construct: EventWriter<'w, TaskStartedEvent<Construct>>,
    pub mine_asteroid: EventWriter<'w, TaskStartedEvent<MineAsteroid>>,
    pub harvest_gas: EventWriter<'w, TaskStartedEvent<HarvestGas>>,
}

/// A [SystemParam] collection of all [TaskCanceledEvent] EventWriters.
/// These are called after a task was removed from the task queue.
#[derive(SystemParam)]
pub struct AllTaskCancelledEventWriters<'w> {
    pub awaiting_signal: EventWriter<'w, TaskCanceledEvent<AwaitingSignal>>,
    pub construct: EventWriter<'w, TaskCanceledEvent<Construct>>,
    pub exchange_wares: EventWriter<'w, TaskCanceledEvent<ExchangeWares>>,
    pub dock_at_entity: EventWriter<'w, TaskCanceledEvent<DockAtEntity>>,
    pub harvest_gas: EventWriter<'w, TaskCanceledEvent<HarvestGas>>,
    pub mine_asteroid: EventWriter<'w, TaskCanceledEvent<MineAsteroid>>,
    pub move_to_entity: EventWriter<'w, TaskCanceledEvent<MoveToEntity>>,
    pub move_to_position: EventWriter<'w, TaskCanceledEvent<MoveToPosition>>,
    pub undock: EventWriter<'w, TaskCanceledEvent<Undock>>,
    pub use_gate: EventWriter<'w, TaskCanceledEvent<UseGate>>,
    pub request_access: EventWriter<'w, TaskCanceledEvent<RequestAccess>>,
}

/// A [SystemParam] collection of all [TaskAbortedEvent] EventWriters.
/// These are called after a task was removed from the task queue.
#[derive(SystemParam)]
pub struct AllTaskAbortedEventWriters<'w> {
    pub awaiting_signal: EventWriter<'w, TaskAbortedEvent<AwaitingSignal>>,
    pub construct: EventWriter<'w, TaskAbortedEvent<Construct>>,
    pub exchange_wares: EventWriter<'w, TaskAbortedEvent<ExchangeWares>>,
    pub dock_at_entity: EventWriter<'w, TaskAbortedEvent<DockAtEntity>>,
    pub harvest_gas: EventWriter<'w, TaskAbortedEvent<HarvestGas>>,
    pub mine_asteroid: EventWriter<'w, TaskAbortedEvent<MineAsteroid>>,
    pub move_to_entity: EventWriter<'w, TaskAbortedEvent<MoveToEntity>>,
    pub move_to_position: EventWriter<'w, TaskAbortedEvent<MoveToPosition>>,
    pub undock: EventWriter<'w, TaskAbortedEvent<Undock>>,
    pub use_gate: EventWriter<'w, TaskAbortedEvent<UseGate>>,
    pub request_access: EventWriter<'w, TaskAbortedEvent<RequestAccess>>,
}

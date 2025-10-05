use crate::components::task_kind::TaskKind;
use crate::impl_all_task_kinds;
use crate::types::entity_wrappers::ShipEntity;
use crate::types::ship_tasks::{
    AwaitingSignal, Construct, DockAtEntity, ExchangeWares, HarvestGas, MineAsteroid, MoveToEntity,
    MoveToPosition, MoveToSector, RequestAccess, ShipTaskData, Undock, UseGate,
};
use bevy::ecs::system::SystemParam;
use bevy::prelude::{Entity, Message, MessageWriter};
use std::marker::PhantomData;

pub mod event_kind {
    pub struct Started;
    pub struct Completed;
    pub struct CanceledWhileActive;
    pub struct CanceledWhileInQueue;
}

/// Indicates that a new active Task was just started.
pub type TaskStartedEvent<T> = TaskEvent<T, event_kind::Started>;
/// Indicates that an active Task was just completed.
pub type TaskCompletedEvent<T> = TaskEvent<T, event_kind::Completed>;

/// Indicates that an active Task was aborted during execution.
/// This usually happens due to entity destruction or user interaction.
pub type TaskCanceledWhileActiveEvent<T> = TaskEventWithData<T, event_kind::CanceledWhileActive>;

/// Indicates that a Task inside the TaskQueue was canceled.
/// This usually happens due to entity destruction or user interaction.
pub type TaskCanceledWhileInQueueEvent<T> = TaskEventWithData<T, event_kind::CanceledWhileInQueue>;

/// Generic base class for all task-related events which don't require to contain a copy of the task data.
/// Use [TaskStartedEvent], [TaskCompletedEvent] for better readability.
#[derive(Message, Copy, Clone)]
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
/// Use [TaskCanceledWhileActiveEvent] and [TaskCanceledWhileInQueueEvent] for better readability.
#[derive(Message, Copy, Clone)]
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
#[derive(Message)]
pub struct InsertTaskIntoQueueCommand<Task: ShipTaskData> {
    /// The entity which should receive the task
    pub entity: Entity,
    /// The task data which should be inserted into the queue.
    pub task_data: Task,
    /// How should the task be inserted?
    pub insertion_mode: TaskInsertionMode,
}

/// Specifies how tasks in [InsertTaskIntoQueueCommand]s should be inserted into the queue.
#[derive(Copy, Clone)]
pub enum TaskInsertionMode {
    /// Appends the tasks to the end of the list
    Append,
    /// Prepends the tasks to the start of the list
    Prepend,
    // TODO: Implementing Replace is a headache I don't want to go through right now
    // /// Clears the entire task queue before adding this new task.
    // Replace,
}

/// Creates TaskMessageWriters and methods for them.
macro_rules! impl_task_events {
    ($(($variant:ident, $snake_case_variant:ident)),*) => {
        /// A [SystemParam] collection of all [TaskStartedEvent] MessageWriters.
        /// Right now this needs to be passed into all behaviors to initiate new tasks.
        #[derive(SystemParam)]
        pub struct AllTaskStartedMessageWriters<'w> {
            $(pub $snake_case_variant: MessageWriter<'w, TaskStartedEvent<$variant>>),*
        }

        impl<'w> AllTaskStartedMessageWriters<'w> {
            /// Writes a [TaskStartedEvent] into the respective [MessageWriter].
            pub fn write_event(&mut self, entity: ShipEntity, task: &TaskKind) {
                match task {
                    $(TaskKind::$variant { .. } => {
                        self.$snake_case_variant.write(TaskStartedEvent::new(entity));
                    }),*
                }
            }
        }

        /// A [SystemParam] collection of all [TaskCanceledWhileInQueueEvent] MessageWriters.
        /// These are called after a task was removed from the task queue.
        #[derive(SystemParam)]
        pub struct AllTaskCancelledMessageWriters<'w> {
            $(pub $snake_case_variant: MessageWriter<'w, TaskCanceledWhileInQueueEvent<$variant>>),*
        }

        impl<'w> AllTaskCancelledMessageWriters<'w> {
            /// Writes a [TaskCanceledWhileInQueueEvent] into the respective [MessageWriter].
            pub fn write_event(&mut self, entity: ShipEntity, task: TaskKind) {
                match task {
                    $(TaskKind::$variant { data } => {
                        self.$snake_case_variant.write(TaskCanceledWhileInQueueEvent::new(entity, data));
                    }),*
                }
            }
        }

        /// A [SystemParam] collection of all [TaskCanceledWhileActiveEvent] MessageWriters.
        /// These are called after a task was removed from the task queue.
        #[derive(SystemParam)]
        pub struct AllTaskAbortedMessageWriters<'w> {
            $(pub $snake_case_variant: MessageWriter<'w, TaskCanceledWhileActiveEvent<$variant>>),*
        }

        impl<'w> AllTaskAbortedMessageWriters<'w> {
            /// Writes a [TaskCanceledWhileInQueueEvent] into the respective [MessageWriter].
            pub fn write_event(&mut self, entity: ShipEntity, task: TaskKind) {
                match task {
                    $(TaskKind::$variant { data } => {
                        self.$snake_case_variant.write(TaskCanceledWhileActiveEvent::new(entity, data));
                    }),*
                }
            }
        }
    };
}

impl_all_task_kinds!(impl_task_events);

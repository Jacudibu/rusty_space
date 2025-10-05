use bevy::prelude::MessageWriter;
use common::events::task_events::TaskCompletedEvent;
use common::types::entity_wrappers::ShipEntity;
use common::types::ship_tasks::ShipTaskData;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::sync::{Arc, Mutex};

pub mod task_cancellation_active;
pub mod task_cancellation_in_queue;
pub mod task_completed;
pub mod task_creation;
pub mod task_started;
pub mod task_update_runner;

/// Default error for blanket implementations for optional TaskTraits.
#[derive(Debug)]
#[allow(dead_code)]
struct TaskTraitFunctionalityNotImplementedError<Task: ShipTaskData> {
    pub entity: ShipEntity,
    pub task_data: Option<Task>,
    pub kind: TaskTraitKind,
}

impl<TaskData: ShipTaskData> Display for TaskTraitFunctionalityNotImplementedError<TaskData> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl<TaskData: ShipTaskData> Error for TaskTraitFunctionalityNotImplementedError<TaskData> {}

/// The kind of trait for [TaskTraitFunctionalityNotImplementedError]
#[derive(Debug)]
#[allow(dead_code)]
enum TaskTraitKind {
    Creation,
    Starting,
    Running,
    CancellationInQueue,
    CancellationWhileActive,
    Completion,
}

/// Unwraps the provided event arc and writes them all at once into the respective [TaskCompletedEvent] event writer.
/// The main idea is that task runners can use par_iter_mut and then just pass any potential event completions in here.
pub fn send_completion_messages<T: ShipTaskData>(
    mut event_writer: MessageWriter<TaskCompletedEvent<T>>,
    task_completions: Arc<Mutex<Vec<TaskCompletedEvent<T>>>>,
) {
    match Arc::try_unwrap(task_completions) {
        Ok(task_completions) => {
            let batch = task_completions.into_inner().unwrap();
            if !batch.is_empty() {
                event_writer.write_batch(batch);
            }
        }
        Err(_) => {
            todo!()
        }
    }
}

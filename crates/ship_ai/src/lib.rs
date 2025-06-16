mod behaviors;
mod plugin;
mod task_lifecycle_traits;
mod tasks;
mod utility;

use bevy::prelude::Component;

/// Marker trait to denote that a struct is used to describe Ship Tasks.
pub trait TaskComponent: Component + Send + Sync + 'static {
    /// Whether this task can be stopped by the user or other means while it is being executed.
    /// Some tasks cannot be aborted because there's no way to stop midway, such as using a gate.
    fn can_be_cancelled_while_active() -> bool;
}

pub use plugin::ShipAiPlugin;
pub use task_lifecycle_traits::task_cancellation_active::TaskCancellationWhileActiveRequest;
pub use task_lifecycle_traits::task_cancellation_active::can_task_be_cancelled_while_active;
pub use task_lifecycle_traits::task_cancellation_in_queue::TaskCancellationWhileInQueueRequest;

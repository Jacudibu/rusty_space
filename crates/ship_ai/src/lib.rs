mod behaviors;
mod plugin;
mod task_lifecycle_traits;
mod tasks;
mod utility;

pub use plugin::ShipAiPlugin;
pub use task_lifecycle_traits::task_cancellation_active::TaskCancellationWhileActiveRequest;
pub use task_lifecycle_traits::task_cancellation_active::can_task_be_cancelled_while_active;
pub use task_lifecycle_traits::task_cancellation_in_queue::TaskCancellationWhileInQueueRequest;

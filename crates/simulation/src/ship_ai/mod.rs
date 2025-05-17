mod behaviors;
mod create_tasks_following_path;
pub mod plugin;
mod ship_task;
mod stop_idle_ships;
pub mod task_cancellation;
mod task_filters;
mod task_result;
mod tasks;
mod trade_plan;

use bevy::prelude::Component;

/// Marker trait to denote that a struct is used to describe Ship Tasks.
pub trait TaskComponent: Component + Send + Sync + 'static {
    /// Whether this task can be stopped by the user or other means while it is being executed.
    /// Some tasks cannot be aborted because there's no way to stop midway, such as using a gate.
    fn can_be_aborted() -> bool;
}

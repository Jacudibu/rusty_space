mod behaviors;
pub mod create_tasks_following_path;
mod plugin;
mod ship_is_idle_filter;
mod stop_idle_ships;
pub mod task_events;
mod task_inside_queue;
mod task_queue;
mod task_result;
mod tasks;

pub use behaviors::BehaviorBuilder;
pub use behaviors::auto_mine::{AutoMineBehavior, AutoMineState};
pub use behaviors::auto_trade::AutoTradeBehavior;
pub use plugin::ShipAiPlugin;
pub use task_events::TaskCompletedEvent;
pub use task_inside_queue::TaskInsideQueue;
pub use task_queue::TaskQueue;
pub use tasks::{AwaitingSignal, Construct};

use bevy::prelude::Component;

/// Marker trait to denote that a struct is used to describe Ship Tasks.
pub trait TaskComponent: Component + Send + Sync + 'static {
    /// Whether this task can be stopped by the user or other means while it is being executed.
    /// Some tasks cannot be aborted because there's no way to stop midway, such as using a gate.
    fn can_be_aborted() -> bool;
}

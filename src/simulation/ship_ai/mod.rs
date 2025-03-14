mod behaviors;
mod plugin;
mod ship_is_idle_filter;
mod stop_idle_ships;
mod task_finished_event;
mod task_inside_queue;
mod task_queue;
mod task_result;
mod task_started_event;
mod tasks;

pub use behaviors::BehaviorBuilder;
pub use behaviors::auto_mine::{AutoMineBehavior, AutoMineState};
pub use behaviors::auto_trade::AutoTradeBehavior;
pub use plugin::ShipAiPlugin;
pub use task_finished_event::TaskFinishedEvent;
pub use task_inside_queue::TaskInsideQueue;
pub use task_queue::TaskQueue;
pub use tasks::{AwaitingSignal, ConstructTaskComponent};

use bevy::prelude::Component;

/// Marker trait to denote that a struct is used to describe Ship Tasks.
pub trait TaskComponent: Component + Send + Sync + 'static {}

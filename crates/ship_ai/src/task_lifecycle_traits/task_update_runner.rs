use bevy::ecs::system::{StaticSystemParam, SystemParam};
use bevy::prelude::EventWriter;
use common::constants::BevyResult;
use common::events::task_events::TaskCompletedEvent;
use common::types::ship_tasks::ShipTaskData;

pub(crate) trait TaskUpdateRunner<'w, 's, Task: ShipTaskData> {
    /// The immutable arguments used when calling the functions of this trait.
    type Args: SystemParam;
    /// The mutable arguments used when calling the functions of this trait.
    type ArgsMut: SystemParam;

    /// Executed once per SimulationTick, this is where the main logic of the task is being executed.
    fn run_all_tasks(
        event_writer: EventWriter<TaskCompletedEvent<Task>>,
        args: StaticSystemParam<Self::Args>,
        args_mut: StaticSystemParam<Self::ArgsMut>,
    ) -> BevyResult;
}

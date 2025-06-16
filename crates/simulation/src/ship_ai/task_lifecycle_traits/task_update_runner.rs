use bevy::ecs::system::{StaticSystemParam, SystemParam};
use bevy::prelude::EventWriter;
use common::constants::BevyResult;
use common::events::task_events::TaskCompletedEvent;
use common::types::ship_tasks::ShipTaskData;

pub(crate) trait TaskUpdateRunner<'w, 's, Task: ShipTaskData> {
    type Args: SystemParam;

    /// Executed once per SimulationTick, this is where the main logic of the task is being executed.
    fn run_all_tasks(
        event_writer: EventWriter<TaskCompletedEvent<Task>>,
        args: StaticSystemParam<Self::Args>,
    ) -> BevyResult;
}

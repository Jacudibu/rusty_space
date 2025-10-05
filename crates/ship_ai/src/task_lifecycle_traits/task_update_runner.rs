use crate::task_lifecycle_traits::send_completion_messages;
use bevy::ecs::system::{StaticSystemParam, SystemParam};
use bevy::prelude::{BevyError, MessageWriter};
use common::constants::BevyResult;
use common::events::task_events::TaskCompletedEvent;
use common::types::ship_tasks::ShipTaskData;
use std::sync::{Arc, Mutex};

pub(crate) trait TaskUpdateRunner<'w, 's, Task: ShipTaskData> {
    /// The immutable arguments used when calling the functions of this trait.
    type Args: SystemParam;
    /// The mutable arguments used when calling the functions of this trait.
    type ArgsMut: SystemParam;

    /// Executed once per SimulationTick, this is where the main logic of the task is being executed.
    /// Tasks are usually executed with par_iter_mut.
    ///
    /// TaskCompletions are happening rather rarely, which is why we use an Arc<Mutex<Vec>> to collect the results.
    ///
    /// # Returns
    /// the [TaskCompletedEvent]s which have been collected during execution.
    fn run_all_tasks(
        args: StaticSystemParam<Self::Args>,
        args_mut: StaticSystemParam<Self::ArgsMut>,
    ) -> Result<Arc<Mutex<Vec<TaskCompletedEvent<Task>>>>, BevyError>;

    /// Executes [Self::run_all_tasks] and sends the completion events that occurred whilst running them.
    ///
    /// Usually you don't need to reimplement this, but if you do, make sure to call [send_completion_messages] at the end!
    fn update(
        event_writer: MessageWriter<TaskCompletedEvent<Task>>,
        args: StaticSystemParam<Self::Args>,
        args_mut: StaticSystemParam<Self::ArgsMut>,
    ) -> BevyResult {
        let completions = Self::run_all_tasks(args, args_mut)?;
        send_completion_messages(event_writer, completions);
        Ok(())
    }
}

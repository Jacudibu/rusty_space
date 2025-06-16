use bevy::ecs::system::{StaticSystemParam, SystemParam};
use bevy::prelude::{BevyError, EventReader};
use common::constants::BevyResult;
use common::events::task_events::TaskCompletedEvent;
use common::types::ship_tasks::ShipTaskData;

/// This trait needs to be implemented for all tasks.
pub(crate) trait TaskCompletedEventHandler<'w, 's, Task: ShipTaskData> {
    /// The immutable arguments used when calling the functions of this trait.
    type Args: SystemParam;
    /// The mutable arguments used when calling the functions of this trait.
    type ArgsMut: SystemParam;

    /// If set to true, the event listener system won't be registered at all. Only do this if there's no custom logic necessary.
    fn skip_completed() -> bool {
        false
    }

    /// You need to either override this or set [Self::skip_completed] to true so the event listener won't be registered.
    fn on_task_completed(
        event: &TaskCompletedEvent<Task>,
        args: &StaticSystemParam<Self::Args>,
        args_mut: &mut StaticSystemParam<Self::ArgsMut>,
    ) -> Result<(), BevyError> {
        todo!("Return a helpful error in case this isn't implemented")
    }

    /// Listens to TaskCancellation Events and runs [Self::on_task_completed] for each.
    /// Usually you don't need to reimplement this.
    fn task_completed_event_listener(
        mut events: EventReader<TaskCompletedEvent<Task>>,
        args: StaticSystemParam<Self::Args>,
        mut args_mut: StaticSystemParam<Self::ArgsMut>,
    ) -> BevyResult {
        for event in events.read() {
            Self::on_task_completed(event, &args, &mut args_mut)?;
        }

        Ok(())
    }
}

use bevy::ecs::system::{StaticSystemParam, SystemParam};
use bevy::prelude::{BevyError, EventReader};
use common::constants::BevyResult;
use common::events::task_events::TaskStartedEvent;
use common::types::ship_tasks::ShipTaskData;

/// This trait needs to be implemented for all tasks.
pub(crate) trait TaskStartedEventHandler<'w, 's, Task: ShipTaskData> {
    /// The immutable arguments used when calling the functions of this trait.
    type Args: SystemParam;
    /// The mutable arguments used when calling the functions of this trait.
    type ArgsMut: SystemParam;

    /// If set to true, the event listener system won't be registered at all. Only do this if there's no custom logic necessary.
    fn skip_started() -> bool {
        false
    }

    /// You need to either override this or set [Self::skip_started] to true so the event listener won't be registered.
    fn on_task_started(
        event: &TaskStartedEvent<Task>,
        args: &StaticSystemParam<Self::Args>,
        args_mut: &mut StaticSystemParam<Self::ArgsMut>,
    ) -> Result<(), BevyError> {
        todo!("Return a helpful error in case this isn't implemented")
    }

    /// Listens to [TaskStartedEvent]s and runs [Self::on_task_started] for each.
    /// Usually you don't need to reimplement this.
    fn task_started_event_listener(
        mut events: EventReader<TaskStartedEvent<Task>>,
        args: StaticSystemParam<Self::Args>,
        mut args_mut: StaticSystemParam<Self::ArgsMut>,
    ) -> BevyResult {
        for event in events.read() {
            Self::on_task_started(event, &args, &mut args_mut)?;
        }

        Ok(())
    }
}

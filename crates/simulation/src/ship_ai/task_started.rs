use bevy::ecs::system::{StaticSystemParam, SystemParam};
use bevy::prelude::{BevyError, EventReader};
use common::constants::BevyResult;
use common::events::task_events::TaskStartedEvent;
use common::types::ship_tasks::ShipTaskData;

/// This trait needs to be implemented for all tasks.
pub(crate) trait TaskStartedEventHandler<Task: ShipTaskData, Args: SystemParam> {
    fn on_task_started(
        event: &TaskStartedEvent<Task>,
        args: &mut StaticSystemParam<Args>,
    ) -> Result<(), BevyError>;

    /// Listens to TaskCancellation Events and runs [Self::on_task_started] for each.
    /// Usually you don't need to reimplement this.
    fn task_started_event_listener(
        mut events: EventReader<TaskStartedEvent<Task>>,
        mut args: StaticSystemParam<Args>,
    ) -> BevyResult {
        for event in events.read() {
            Self::on_task_started(event, &mut args)?;
        }

        Ok(())
    }
}

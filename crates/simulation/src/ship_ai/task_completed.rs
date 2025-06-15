use bevy::ecs::system::{StaticSystemParam, SystemParam};
use bevy::prelude::{BevyError, EventReader};
use common::constants::BevyResult;
use common::events::task_events::TaskCompletedEvent;
use common::types::ship_tasks::ShipTaskData;

/// This trait needs to be implemented for all tasks.
pub(crate) trait TaskCompletedEventHandler<Task: ShipTaskData, Args: SystemParam> {
    fn on_task_completed(
        event: &TaskCompletedEvent<Task>,
        args: &mut StaticSystemParam<Args>,
    ) -> Result<(), BevyError>;

    /// Listens to TaskCancellation Events and runs [Self::on_task_completed] for each.
    /// Usually you don't need to reimplement this.
    fn task_completed_event_listener(
        mut events: EventReader<TaskCompletedEvent<Task>>,
        mut args: StaticSystemParam<Args>,
    ) -> BevyResult {
        for event in events.read() {
            Self::on_task_completed(event, &mut args)?;
        }

        Ok(())
    }
}

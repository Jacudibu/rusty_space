use crate::task_lifecycle_traits::{TaskTraitFunctionalityNotImplementedError, TaskTraitKind};
use crate::tasks::apply_next_task;
use crate::utility::ship_task::ShipTask;
use bevy::ecs::system::{StaticSystemParam, SystemParam};
use bevy::log::error;
use bevy::prelude::{BevyError, Commands, EventReader, Query, With};
use common::components::task_queue::TaskQueue;
use common::constants::BevyResult;
use common::events::task_events::{AllTaskStartedEventWriters, TaskCompletedEvent};
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
        _args: &StaticSystemParam<Self::Args>,
        _args_mut: &mut StaticSystemParam<Self::ArgsMut>,
    ) -> Result<(), BevyError> {
        Err(BevyError::from(
            TaskTraitFunctionalityNotImplementedError::<Task> {
                entity: event.entity,
                task_data: None,
                kind: TaskTraitKind::CancellationWhileActive,
            },
        ))
    }

    /// Listens to [TaskCompletedEvent]s and runs [Self::on_task_completed] for each.
    ///
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

    /// Listens to [TaskCompletedEvent]s and ensures that the next task is started properly.
    /// This includes removing the existing TaskComponent and adding a new one.
    ///
    /// Usually you don't need to reimplement this.
    fn remove_completed_task_and_start_next_one(
        mut commands: Commands,
        mut event_reader: EventReader<TaskCompletedEvent<Task>>,
        mut all_ships_with_task: Query<&mut TaskQueue, With<ShipTask<Task>>>,
        mut task_started_event_writers: AllTaskStartedEventWriters,
    ) {
        for event in event_reader.read() {
            if let Ok(mut queue) = all_ships_with_task.get_mut(event.entity.into()) {
                let entity = event.entity.into();
                let mut entity_commands = commands.entity(entity);
                entity_commands.remove::<ShipTask<Task>>();
                apply_next_task(
                    &mut queue,
                    entity.into(),
                    &mut entity_commands,
                    &mut task_started_event_writers,
                );
            } else {
                error!(
                    "Unable to find entity for generic task completion: {}",
                    event.entity
                );
            }
        }
    }
}

use crate::TaskKindExt;
use crate::task_lifecycle_traits::{
    TaskTraitFunctionalityNotImplementedError, TaskTraitKind, task_cancellation_in_queue,
};
use crate::utility::ship_task::ShipTask;
use bevy::ecs::system::{StaticSystemParam, SystemParam};
use bevy::prelude::{BevyError, Commands, Event, EventReader, Query, info, warn};
use common::components::task_queue::TaskQueue;
use common::constants::BevyResult;
use common::events::task_events::{
    AllTaskAbortedEventWriters, AllTaskCancelledEventWriters, TaskCanceledWhileActiveEvent,
};
use common::types::entity_wrappers::ShipEntity;
use common::types::ship_tasks::{
    DockAtEntity, ExchangeWares, HarvestGas, MineAsteroid, MoveToEntity, MoveToPosition,
    MoveToSector, RequestAccess, ShipTaskData, Undock, UseGate,
};

/// Send this event in order to request a ship to stop doing whatever it is doing right now, and also clear its entire task queue.
/// Tasks which are aborted are also getting cancelled, so there's no reason to implement cancellation logic within the abortion handler.
#[derive(Event)]
pub struct TaskCancellationWhileActiveRequest {
    /// The affected entity.
    pub entity: ShipEntity,
}

/// This trait needs to be implemented for all tasks.
///
/// Provides a blanket implementation which prints errors in case we attempt to cancel a task which
/// is not supposed to be cancelled
pub(crate) trait TaskCancellationForActiveTaskEventHandler<'w, 's, TaskData: ShipTaskData> {
    /// The immutable arguments used when calling the functions of this trait.
    type Args: SystemParam;
    /// The mutable arguments used when calling the functions of this trait.
    type ArgsMut: SystemParam;

    /// Whether this task may be cancelled while it is actively being executed.
    fn can_task_be_cancelled_while_active() -> bool {
        false
    }

    /// If set to true, the event listener system won't be registered at all. Only do this if there's no custom logic necessary.
    fn skip_cancelled_while_active() -> bool {
        true
    }

    /// You need to either override this or set [Self::skip_cancelled_while_active] to true so the event listener won't be registered.
    fn on_task_cancellation_while_in_active(
        event: &TaskCanceledWhileActiveEvent<TaskData>,
        _args: &StaticSystemParam<Self::Args>,
        _args_mut: &mut StaticSystemParam<Self::ArgsMut>,
    ) -> Result<(), BevyError> {
        Err(BevyError::from(TaskTraitFunctionalityNotImplementedError {
            entity: event.entity,
            task_data: event.task_data.clone().into(),
            kind: TaskTraitKind::CancellationWhileActive,
        }))
    }

    /// Listens to [TaskCanceledWhileActiveEvent]s and runs [Self::on_task_cancellation_while_in_active] for each.
    /// Usually you don't need to reimplement this.
    fn cancellation_while_active_event_listener(
        mut commands: Commands,
        mut events: EventReader<TaskCanceledWhileActiveEvent<TaskData>>,
        args: StaticSystemParam<Self::Args>,
        mut args_mut: StaticSystemParam<Self::ArgsMut>,
    ) -> BevyResult {
        for event in events.read() {
            Self::on_task_cancellation_while_in_active(event, &args, &mut args_mut)?;
            commands
                .entity(event.entity.into())
                .remove::<ShipTask<TaskData>>();
        }

        Ok(())
    }
}

/// Completely clears task queues.
pub(crate) fn handle_task_cancellation_while_active_requests(
    mut events: EventReader<TaskCancellationWhileActiveRequest>,
    mut all_task_queues: Query<&mut TaskQueue>,
    mut all_task_cancelled_while_active_event_writers: AllTaskAbortedEventWriters,
    mut all_task_cancelled_while_in_queue_event_writers: AllTaskCancelledEventWriters,
) -> BevyResult {
    for event in events.read() {
        let mut queue = all_task_queues.get_mut(event.entity.into())?;

        let Some(active_task) = &queue.active_task else {
            info!(
                "Abort task was called on an entity without active task: {}",
                event.entity
            );
            continue;
        };

        if !active_task.can_task_be_cancelled_while_active() {
            warn!(
                "Abort task was called on an entity with a task which cannot be aborted: {}",
                event.entity
            );
            continue;
        }

        all_task_cancelled_while_active_event_writers
            .write_event(event.entity, active_task.clone());
        all_task_cancelled_while_in_queue_event_writers
            .write_event(event.entity, active_task.clone());

        for task in queue.queue.split_off(0) {
            all_task_cancelled_while_in_queue_event_writers.write_event(event.entity, task);
        }

        queue.active_task = None;
    }

    Ok(())
}

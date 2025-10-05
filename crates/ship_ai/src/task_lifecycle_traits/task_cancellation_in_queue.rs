use crate::TaskCancellationWhileActiveRequest;
use crate::task_lifecycle_traits::{TaskTraitFunctionalityNotImplementedError, TaskTraitKind};
use bevy::ecs::system::{StaticSystemParam, SystemParam};
use bevy::prelude::{BevyError, Message, MessageReader, MessageWriter, Query};
use common::components::task_kind::TaskKind;
use common::components::task_queue::TaskQueue;
use common::constants::BevyResult;
use common::events::task_events::{AllTaskCancelledMessageWriters, TaskCanceledWhileInQueueEvent};
use common::types::entity_wrappers::ShipEntity;
use common::types::ship_tasks::ShipTaskData;

/// Send this event in order to request removing tasks from a task queue.
#[derive(Message)]
pub struct TaskCancellationWhileInQueueRequest {
    /// The affected entity.
    pub entity: ShipEntity,
    /// The index of the task which should be cancelled. This and all following tasks will be removed.
    pub task_position_in_queue: usize,
}

/// This trait needs to be implemented for all tasks.
///
/// Provides a blanket implementation which prints errors in case we attempt to cancel a task which
/// is not supposed to be cancelled
pub(crate) trait TaskCancellationForTaskInQueueEventHandler<'w, 's, TaskData: ShipTaskData> {
    /// The immutable arguments used when calling the functions of this trait.
    type Args: SystemParam;
    /// The mutable arguments used when calling the functions of this trait.
    type ArgsMut: SystemParam;

    /// If set to true, the event listener system won't be registered at all. Only do this if there's no custom logic necessary.
    fn skip_cancelled_in_queue() -> bool {
        false
    }

    /// You need to either override this or set [Self::skip_cancelled_in_queue] to true so the event listener won't be registered.
    fn on_task_cancellation_while_in_queue(
        event: &TaskCanceledWhileInQueueEvent<TaskData>,
        _args: &StaticSystemParam<Self::Args>,
        _args_mut: &mut StaticSystemParam<Self::ArgsMut>,
    ) -> Result<(), BevyError> {
        Err(BevyError::from(TaskTraitFunctionalityNotImplementedError {
            entity: event.entity,
            task_data: event.task_data.clone().into(),
            kind: TaskTraitKind::CancellationInQueue,
        }))
    }

    /// Listens to [TaskCancellationWhileInQueueEvent]s and runs [Self::on_task_cancellation_while_in_queue] for each.
    /// Usually you don't need to reimplement this.
    fn cancellation_while_in_queue_event_listener(
        mut events: MessageReader<TaskCanceledWhileInQueueEvent<TaskData>>,
        args: StaticSystemParam<Self::Args>,
        mut args_mut: StaticSystemParam<Self::ArgsMut>,
    ) -> BevyResult {
        for event in events.read() {
            Self::on_task_cancellation_while_in_queue(event, &args, &mut args_mut)?;
        }

        Ok(())
    }
}

pub(crate) fn handle_task_cancellation_while_in_queue_requests(
    mut events: MessageReader<TaskCancellationWhileInQueueRequest>,
    mut all_task_queues: Query<&mut TaskQueue>,
    mut event_writers: AllTaskCancelledMessageWriters,
    mut task_abortion_request_writer: MessageWriter<TaskCancellationWhileActiveRequest>,
) -> BevyResult {
    for event in events.read() {
        let mut queue = all_task_queues.get_mut(event.entity.into())?;

        if event.task_position_in_queue == 0
            && matches!(queue.active_task, Some(TaskKind::RequestAccess { .. }))
        {
            // RequestAccess makes things a little iffy here, but that'll resolve itself once we use entity relationships for this
            task_abortion_request_writer.write(TaskCancellationWhileActiveRequest {
                entity: event.entity,
            });
            continue;
        }

        let split_position = if event.task_position_in_queue > 0 {
            if matches!(
                queue.queue.get(event.task_position_in_queue - 1),
                Some(TaskKind::RequestAccess { .. })
            ) {
                // RequestAccess also needs to be cancelled, else the affected queue will be *very* sad.
                event.task_position_in_queue - 1
            } else {
                event.task_position_in_queue
            }
        } else {
            event.task_position_in_queue
        };

        for task in queue.queue.split_off(split_position) {
            event_writers.write_event(event.entity, task);
        }
    }

    Ok(())
}

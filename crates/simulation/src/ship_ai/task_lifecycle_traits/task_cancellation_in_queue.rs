use crate::TaskCancellationWhileActiveRequest;
use bevy::ecs::system::{StaticSystemParam, SystemParam};
use bevy::prelude::{BevyError, Event, EventReader, EventWriter, Query};
use common::components::task_kind::TaskKind;
use common::components::task_queue::TaskQueue;
use common::constants::BevyResult;
use common::events::task_events::{AllTaskCancelledEventWriters, TaskCanceledWhileInQueueEvent};
use common::types::entity_wrappers::ShipEntity;
use common::types::ship_tasks::ShipTaskData;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

/// Send this event in order to request removing tasks from a task queue.
#[derive(Event)]
pub struct TaskCancellationWhileInQueueRequest {
    /// The affected entity.
    pub entity: ShipEntity,
    /// The index of the task which should be cancelled. This and all following tasks will be removed.
    pub task_position_in_queue: usize,
}

/// Default error used during [TaskCancellationForTaskInQueueEventHandler].
#[derive(Debug)]
struct TaskCancellationInQueueNotImplementedError<TaskData: ShipTaskData> {
    entity: ShipEntity,
    task_data: TaskData,
}

impl<TaskData: ShipTaskData> Display for TaskCancellationInQueueNotImplementedError<TaskData> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl<TaskData: ShipTaskData> Error for TaskCancellationInQueueNotImplementedError<TaskData> {}

/// This trait needs to be implemented for all tasks.
///
/// Provides a blanket implementation which prints errors in case we attempt to cancel a task which
/// is not supposed to be cancelled
pub(crate) trait TaskCancellationForTaskInQueueEventHandler<'w, 's, TaskData: ShipTaskData> {
    /// The immutable arguments used when calling the functions of this trait.
    type Args: SystemParam;
    /// The mutable arguments used when calling the functions of this trait.
    type ArgsMut: SystemParam;

    /// Whether this task can be cancelled while it is still in the queue
    fn can_task_be_cancelled_while_in_queue() -> bool {
        false
    }

    fn on_task_cancellation_while_in_queue(
        event: &TaskCanceledWhileInQueueEvent<TaskData>,
        _args: &StaticSystemParam<Self::Args>,
        _args_mut: &mut StaticSystemParam<Self::ArgsMut>,
    ) -> Result<(), BevyError> {
        Err(BevyError::from(
            TaskCancellationInQueueNotImplementedError {
                entity: event.entity,
                task_data: event.task_data.clone(),
            },
        ))
    }

    /// Listens to TaskCancellation Events and runs [Self::on_task_cancellation_while_in_queue] for each.
    /// Usually you don't need to reimplement this.
    fn cancellation_while_in_queue_event_listener(
        mut events: EventReader<TaskCanceledWhileInQueueEvent<TaskData>>,
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
    mut events: EventReader<TaskCancellationWhileInQueueRequest>,
    mut all_task_queues: Query<&mut TaskQueue>,
    mut event_writers: AllTaskCancelledEventWriters,
    mut task_abortion_request_writer: EventWriter<TaskCancellationWhileActiveRequest>,
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
            send_cancellation_event(&mut event_writers, event.entity, task);
        }
    }

    Ok(())
}

pub(crate) fn send_cancellation_event(
    event_writers: &mut AllTaskCancelledEventWriters,
    entity: ShipEntity,
    task: TaskKind,
) {
    match task {
        TaskKind::AwaitingSignal { data } => {
            write_event(&mut event_writers.awaiting_signal, entity, data)
        }
        TaskKind::Construct { data } => {
            write_event(&mut event_writers.construct, entity, data);
        }
        TaskKind::RequestAccess { data } => {
            write_event(&mut event_writers.request_access, entity, data)
        }
        TaskKind::DockAtEntity { data } => {
            write_event(&mut event_writers.dock_at_entity, entity, data)
        }
        TaskKind::Undock { data } => write_event(&mut event_writers.undock, entity, data),
        TaskKind::ExchangeWares { data } => {
            write_event(&mut event_writers.exchange_wares, entity, data)
        }
        TaskKind::MoveToEntity { data } => {
            write_event(&mut event_writers.move_to_entity, entity, data)
        }
        TaskKind::MoveToPosition { data } => {
            write_event(&mut event_writers.move_to_position, entity, data)
        }
        TaskKind::MoveToSector { data } => {
            write_event(&mut event_writers.move_to_sector, entity, data)
        }
        TaskKind::UseGate { data } => write_event(&mut event_writers.use_gate, entity, data),
        TaskKind::MineAsteroid { data } => {
            write_event(&mut event_writers.mine_asteroid, entity, data)
        }
        TaskKind::HarvestGas { data } => write_event(&mut event_writers.harvest_gas, entity, data),
    }
}

#[inline]
fn write_event<T: ShipTaskData>(
    event_writer: &mut EventWriter<TaskCanceledWhileInQueueEvent<T>>,
    entity: ShipEntity,
    data: T,
) {
    event_writer.write(TaskCanceledWhileInQueueEvent::new(entity, data));
}

use crate::TaskAbortionRequest;
use bevy::prelude::{Event, EventReader, EventWriter, Query};
use common::components::task_kind::TaskKind;
use common::components::task_queue::TaskQueue;
use common::constants::BevyResult;
use common::events::task_events::{AllTaskCancelledEventWriters, TaskCanceledEvent};
use common::types::entity_wrappers::ShipEntity;
use common::types::ship_tasks::ShipTaskData;
use std::collections::VecDeque;

/// Send this event in order to request removing tasks from a task queue.
#[derive(Event)]
pub struct TaskCancellationRequest {
    /// The affected entity.
    pub entity: ShipEntity,
    /// The index of the task which should be cancelled. This and all following tasks will be removed.
    pub task_position_in_queue: usize,
}

pub(crate) fn handle_task_cancellation_requests(
    mut events: EventReader<TaskCancellationRequest>,
    mut all_task_queues: Query<&mut TaskQueue>,
    mut event_writers: AllTaskCancelledEventWriters,
    mut task_abortion_request_writer: EventWriter<TaskAbortionRequest>,
) -> BevyResult {
    for event in events.read() {
        let mut queue = all_task_queues.get_mut(event.entity.into())?;

        if event.task_position_in_queue == 0
            && matches!(queue.active_task, Some(TaskKind::RequestAccess { .. }))
        {
            // RequestAccess makes things a little iffy here, but that'll resolve itself once we use entity relationships for this
            task_abortion_request_writer.write(TaskAbortionRequest {
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
        TaskKind::UseGate { data } => write_event(&mut event_writers.use_gate, entity, data),
        TaskKind::MineAsteroid { data } => {
            write_event(&mut event_writers.mine_asteroid, entity, data)
        }
        TaskKind::HarvestGas { data } => write_event(&mut event_writers.harvest_gas, entity, data),
    }
}

#[inline]
fn write_event<T: ShipTaskData + 'static>(
    event_writer: &mut EventWriter<TaskCanceledEvent<T>>,
    entity: ShipEntity,
    data: T,
) {
    event_writer.write(TaskCanceledEvent::new(entity, data));
}

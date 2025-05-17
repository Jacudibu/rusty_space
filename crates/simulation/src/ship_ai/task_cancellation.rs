use bevy::prelude::{Event, EventReader, EventWriter, Query};
use common::components::task_kind::TaskKind;
use common::components::task_queue::TaskQueue;
use common::constants::BevyResult;
use common::events::task_events::{AllTaskCancelledEventWriters, TaskCanceledEvent};
use common::types::entity_wrappers::ShipEntity;
use common::types::ship_tasks::ShipTaskData;

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
) -> BevyResult {
    for event in events.read() {
        let mut queue = all_task_queues.get_mut(event.entity.into())?;

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

        let cancellations = queue.queue.split_off(split_position);
        for task in cancellations {
            match task {
                TaskKind::AwaitingSignal { data } => {
                    write_event(&mut event_writers.awaiting_signal, event.entity, data)
                }
                TaskKind::Construct { data } => {
                    write_event(&mut event_writers.construct, event.entity, data);
                }
                TaskKind::RequestAccess { data } => {
                    write_event(&mut event_writers.request_access, event.entity, data)
                }
                TaskKind::DockAtEntity { data } => {
                    write_event(&mut event_writers.dock_at_entity, event.entity, data)
                }
                TaskKind::Undock { data } => {
                    write_event(&mut event_writers.undock, event.entity, data)
                }
                TaskKind::ExchangeWares { data } => {
                    write_event(&mut event_writers.exchange_wares, event.entity, data)
                }
                TaskKind::MoveToEntity { data } => {
                    write_event(&mut event_writers.move_to_entity, event.entity, data)
                }
                TaskKind::UseGate { data } => {
                    write_event(&mut event_writers.use_gate, event.entity, data)
                }
                TaskKind::MineAsteroid { data } => {
                    write_event(&mut event_writers.mine_asteroid, event.entity, data)
                }
                TaskKind::HarvestGas { data } => {
                    write_event(&mut event_writers.harvest_gas, event.entity, data)
                }
            }
        }
    }

    Ok(())
}

#[inline]
fn write_event<T: ShipTaskData + 'static>(
    event_writer: &mut EventWriter<TaskCanceledEvent<T>>,
    entity: ShipEntity,
    data: T,
) {
    event_writer.write(TaskCanceledEvent::new(entity, data));
}

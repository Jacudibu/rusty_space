use crate::ship_ai::task_cancellation;
use bevy::prelude::{Event, EventReader, EventWriter, Query};
use common::components::task_kind::TaskKind;
use common::components::task_queue::TaskQueue;
use common::constants::BevyResult;
use common::events::task_events::{
    AllTaskAbortedEventWriters, AllTaskCancelledEventWriters, TaskAbortedEvent,
};
use common::types::entity_wrappers::ShipEntity;
use common::types::ship_tasks::ShipTaskData;

/// Send this event in order to request a ship to stop doing whatever it is doing right now, and also clear its entire task queue.
/// Tasks which are aborted are also getting cancelled, so there's no reason to implement cancellation logic within the abortion handler.
#[derive(Event)]
pub struct TaskAbortionRequest {
    /// The affected entity.
    pub entity: ShipEntity,
}

/// Completely clears task queues.
pub(crate) fn handle_task_abortion_requests(
    mut events: EventReader<TaskAbortionRequest>,
    mut all_task_queues: Query<&mut TaskQueue>,
    mut event_writers: AllTaskAbortedEventWriters,
    mut cancellation_event_writers: AllTaskCancelledEventWriters,
) -> BevyResult {
    for event in events.read() {
        let mut queue = all_task_queues.get_mut(event.entity.into())?;

        // TODO: Figure out whether active task may be aborted.

        match queue.active_task.clone().unwrap() {
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
            TaskKind::Undock { data } => write_event(&mut event_writers.undock, event.entity, data),
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

        task_cancellation::send_cancellation_event(
            &mut cancellation_event_writers,
            event.entity,
            queue.active_task.clone().unwrap(),
        );
        for task in queue.queue.split_off(0) {
            task_cancellation::send_cancellation_event(
                &mut cancellation_event_writers,
                event.entity,
                task,
            );
        }

        queue.active_task = None;
    }

    Ok(())
}

#[inline]
fn write_event<T: ShipTaskData + Clone + 'static>(
    event_writer: &mut EventWriter<TaskAbortedEvent<T>>,
    entity: ShipEntity,
    data: T,
) {
    event_writer.write(TaskAbortedEvent::new(entity, data));
}

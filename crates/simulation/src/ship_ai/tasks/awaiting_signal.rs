use crate::ship_ai::TaskComponent;
use crate::ship_ai::ship_task::ShipTask;
use bevy::prelude::{EventReader, Query};
use common::components::interaction_queue::InteractionQueue;
use common::constants::BevyResult;
use common::events::task_events::TaskCanceledWhileActiveEvent;
use common::types::ship_tasks::AwaitingSignal;

impl TaskComponent for ShipTask<AwaitingSignal> {
    fn can_be_cancelled_while_active() -> bool {
        true
    }
}

pub(crate) fn cancel_task_inside_queue() {
    // Nothing needs to be done.
}

pub(crate) fn abort_running_task(
    mut cancelled_tasks: EventReader<TaskCanceledWhileActiveEvent<AwaitingSignal>>,
    mut interaction_queues: Query<&mut InteractionQueue>,
) {
    for event in cancelled_tasks.read() {
        if let Ok(mut interaction_queue) = interaction_queues.get_mut(event.task_data.from.into()) {
            interaction_queue.remove_from_queue(event.entity);
        }
    }
}

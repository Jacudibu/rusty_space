use crate::ship_ai::TaskComponent;
use crate::ship_ai::ship_task::ShipTask;
use bevy::prelude::{EventReader, Query, With};
use common::components::Ship;
use common::components::interaction_queue::InteractionQueue;
use common::constants::BevyResult;
use common::events::task_events::TaskAbortedEvent;
use common::types::ship_tasks::AwaitingSignal;

impl TaskComponent for ShipTask<AwaitingSignal> {
    fn can_be_aborted() -> bool {
        true
    }
}

pub(crate) fn cancel_task_inside_queue() {
    // Nothing needs to be done.
}

pub(crate) fn abort_tasks(
    mut cancelled_tasks: EventReader<TaskAbortedEvent<AwaitingSignal>>,
    ships: Query<&ShipTask<AwaitingSignal>, With<Ship>>,
    mut interaction_queues: Query<&mut InteractionQueue>,
) -> BevyResult {
    for event in cancelled_tasks.read() {
        let task = ships.get(event.entity.into())?;
        let mut interaction_queue = interaction_queues.get_mut(task.from.into())?;
        interaction_queue.remove_from_queue(event.entity);
    }

    Ok(())
}

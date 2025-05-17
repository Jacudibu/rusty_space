use crate::ship_ai::TaskComponent;
use crate::ship_ai::ship_task::ShipTask;
use bevy::prelude::{Entity, EventWriter, Query, info, warn};
use common::components::interaction_queue::InteractionQueue;
use common::components::task_kind::TaskKind;
use common::components::task_queue::TaskQueue;
use common::events::task_events::TaskCompletedEvent;
use common::types::ship_tasks::{AwaitingSignal, RequestAccess};

impl TaskComponent for ShipTask<RequestAccess> {
    fn can_be_aborted() -> bool {
        true
    }
}

impl ShipTask<RequestAccess> {
    pub fn run_tasks(
        mut all_ships_with_task: Query<(Entity, &Self, &mut TaskQueue)>,
        mut all_interaction_queues: Query<&mut InteractionQueue>,
        mut task_completions: EventWriter<TaskCompletedEvent<RequestAccess>>,
    ) {
        for (entity, task, mut task_queue) in all_ships_with_task.iter_mut() {
            let Ok(mut interaction_queue) = all_interaction_queues.get_mut(task.target.into())
            else {
                // TODO: Cancel dependant tasks
                warn!(
                    "Unable to find target entity for {:?}'s request_access task!",
                    entity
                );
                // TODO: Right now we know that the next three tasks are dock, sell, undock,
                //       but that's not always going to be a given and needs to be handled properly
                let request_access_task = task_queue.queue.pop_front().unwrap();
                task_queue.queue.pop_front();
                task_queue.queue.pop_front();
                task_queue.queue.pop_front();
                task_queue.push_front(request_access_task);
                task_completions.write(TaskCompletedEvent::<RequestAccess>::new(entity.into()));
                continue;
            };

            if interaction_queue
                .try_start_interaction(entity.into())
                .is_err()
            {
                task_queue.push_front(TaskKind::AwaitingSignal {
                    data: AwaitingSignal { from: task.target },
                });
            }

            task_completions.write(TaskCompletedEvent::<RequestAccess>::new(entity.into()));
        }
    }

    pub(crate) fn cancel_task_inside_queue() {
        // Nothing needs to be done
    }
}

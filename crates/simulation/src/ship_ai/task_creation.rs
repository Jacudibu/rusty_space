pub(crate) mod move_to_position;

use crate::can_task_be_cancelled_while_active;
use crate::ship_ai::tasks::apply_next_task;
use bevy::ecs::system::{StaticSystemParam, SystemParam};
use bevy::log::warn;
use bevy::prelude::{BevyError, Commands, Entity, EventReader, Mut};
use common::components::task_kind::TaskKind;
use common::components::task_queue::TaskQueue;
use common::events::task_events::{
    AllTaskStartedEventWriters, InsertTaskIntoQueueCommand, TaskInsertionMode,
};
use common::types::ship_tasks::ShipTaskData;
use std::collections::VecDeque;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub(crate) trait TaskCreation<TaskData: ShipTaskData + 'static, Args: SystemParam> {
    fn handle_creation_command(
        event: &InsertTaskIntoQueueCommand<TaskData>,
        args: &mut StaticSystemParam<Args>,
    ) -> Result<(), BevyError>;
}

#[derive(Debug)]
pub(crate) struct TaskCreationError {
    entity: Entity,
    reason: TaskCreationErrorReason,
}

impl Display for TaskCreationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for TaskCreationError {}

#[derive(Debug)]
pub(crate) enum TaskCreationErrorReason {
    ShipNotFound,
}

pub(crate) fn create_task_command_listener<TaskData, Args>(
    mut events: EventReader<InsertTaskIntoQueueCommand<TaskData>>,
    mut args: StaticSystemParam<Args>,
) where
    TaskData: ShipTaskData + TaskCreation<TaskData, Args> + 'static,
    Args: SystemParam,
{
    for event in events.read() {
        if let Err(e) = TaskData::handle_creation_command(event, &mut args) {
            warn!(
                "Error whilst running move_to_position_command_listener: {:?}",
                e
            );
        }
    }
}

/// Applies the provided list of Tasks to the provided TaskQueue. Should be called at the end of CreateTaskCommand Listeners.
fn apply_tasks(
    mut new_tasks: VecDeque<TaskKind>,
    task_insertion_mode: TaskInsertionMode,
    entity: Entity,
    queue: &mut Mut<TaskQueue>,
    all_task_started_event_writers: &mut AllTaskStartedEventWriters,
    commands: &mut Commands,
) {
    match task_insertion_mode {
        TaskInsertionMode::Append => {
            queue.append(&mut new_tasks);
        }
        TaskInsertionMode::Prepend => {
            if let Some(active_task) = queue.active_task.clone() {
                if !can_task_be_cancelled_while_active(&active_task) {
                    todo!(
                        "So, uh, this should probably just be skipped! Ideally before we do all the earlier calculation."
                    )
                }

                todo!(
                    "This isn't an abortion... and also not a cancellation... so, uh... yet another event to handle? Yaaay...~\
                         On the bright side, such task-delay thingies are inevitable in case a ship gets attacked and has to escape later on.
                         We also need to remove the TaskComponent from the entity... fun!\
                         Best way is probably to leave the active task as-is and do all that in the task-delayed event handler. This is gonna be easier with task grouping."
                );
                queue.active_task = None;
                queue.push_front(active_task);
            }

            for x in new_tasks.into_iter().rev() {
                queue.push_front(x);
            }
        }
    };

    if queue.active_task.is_none() {
        apply_next_task(
            queue,
            entity.into(),
            &mut commands.entity(entity),
            all_task_started_event_writers,
        )
    }
}

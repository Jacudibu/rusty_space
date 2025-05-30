use crate::can_task_be_cancelled_while_active;
use crate::ship_ai::tasks::apply_next_task;
use bevy::ecs::system::{StaticSystemParam, SystemParam};
use bevy::log::warn;
use bevy::prelude::{BevyError, Commands, Entity, EventReader, Mut, Query};
use common::components::task_kind::TaskKind;
use common::components::task_queue::TaskQueue;
use common::constants::BevyResult;
use common::events::task_events::{
    AllTaskStartedEventWriters, InsertTaskIntoQueueCommand, TaskInsertionMode,
};
use common::types::ship_tasks::ShipTaskData;
use std::collections::VecDeque;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub(crate) trait TaskCreation<TaskData: ShipTaskData + 'static, Args: SystemParam> {
    /// Creates a VecDequeue with all subtasks necessary to achieve this Tasks.
    fn create_tasks_for_command(
        event: &InsertTaskIntoQueueCommand<TaskData>,
        args: &StaticSystemParam<Args>,
    ) -> Result<VecDeque<TaskKind>, BevyError>;
}

#[derive(Debug)]
/// Error Type used during TaskCreation.
pub(crate) struct TaskCreationError {
    pub entity: Entity,
    pub reason: TaskCreationErrorReason,
}

impl Display for TaskCreationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for TaskCreationError {}

#[derive(Debug)]
/// An enum to further explain what went wrong during task creation.
pub(crate) enum TaskCreationErrorReason {
    ShipNotFound,
}

pub(crate) fn create_task_command_listener<TaskData, Args>(
    mut events: EventReader<InsertTaskIntoQueueCommand<TaskData>>,
    args_for_creation: StaticSystemParam<Args>,
    mut all_task_queues: Query<&mut TaskQueue>,
    mut commands: Commands,
    mut all_task_started_event_writers: AllTaskStartedEventWriters,
) -> BevyResult
where
    TaskData: ShipTaskData + TaskCreation<TaskData, Args> + 'static,
    Args: SystemParam,
{
    for event in events.read() {
        match TaskData::create_tasks_for_command(event, &args_for_creation) {
            Err(e) => {
                warn!(
                    "Error whilst running move_to_position_command_listener: {:?}",
                    e
                );
            }
            Ok(tasks) => {
                apply_tasks(
                    tasks,
                    event.insertion_mode,
                    event.entity,
                    all_task_queues.get_mut(event.entity)?, // Since creation didn't fail, this shouldn't fail either.
                    &mut all_task_started_event_writers,
                    commands.reborrow(),
                );
            }
        }
    }

    Ok(())
}

/// Applies the provided list of Tasks to the provided TaskQueue. Should be called at the end of CreateTaskCommand Listeners.
fn apply_tasks(
    mut new_tasks: VecDeque<TaskKind>,
    task_insertion_mode: TaskInsertionMode,
    entity: Entity,
    mut queue: Mut<TaskQueue>,
    all_task_started_event_writers: &mut AllTaskStartedEventWriters,
    mut commands: Commands,
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
            &mut queue,
            entity.into(),
            &mut commands.entity(entity),
            all_task_started_event_writers,
        )
    }
}

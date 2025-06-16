use crate::can_task_be_cancelled_while_active;
use crate::tasks::apply_next_task;
use bevy::ecs::system::{StaticSystemParam, SystemParam};
use bevy::log::warn;
use bevy::prelude::{BevyError, Commands, Entity, EventReader, Query, Transform};
use common::components::task_kind::TaskKind;
use common::components::task_queue::TaskQueue;
use common::components::{InSector, IsDocked, Sector};
use common::constants::BevyResult;
use common::events::task_events::{
    AllTaskStartedEventWriters, InsertTaskIntoQueueCommand, TaskInsertionMode,
};
use common::simulation_transform::SimulationTransform;
use common::types::ship_tasks::ShipTaskData;
use std::collections::VecDeque;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub(crate) trait TaskCreationEventHandler<'w, 's, TaskData: ShipTaskData> {
    /// The immutable arguments used when calling the functions of this trait.
    type Args: SystemParam;
    /// The mutable arguments used when calling the functions of this trait.
    type ArgsMut: SystemParam;

    /// Creates a VecDequeue with all subtasks necessary to achieve this Task.
    fn create_tasks_for_command(
        event: &InsertTaskIntoQueueCommand<TaskData>,
        task_queue: &TaskQueue,
        general_pathfinding_args: &GeneralPathfindingArgs,
        args: &StaticSystemParam<Self::Args>,
        args_mut: &mut StaticSystemParam<Self::ArgsMut>,
    ) -> Result<VecDeque<TaskKind>, BevyError>;

    /// Listens to [InsertTaskIntoQueueCommand]<TaskData> Events and runs [Self::create_tasks_for_command] for each.
    /// Usually you don't need to reimplement this.
    fn task_creation_event_listener(
        mut events: EventReader<InsertTaskIntoQueueCommand<TaskData>>,
        general_pathfinding_args: GeneralPathfindingArgs,
        args: StaticSystemParam<Self::Args>,
        mut args_mut: StaticSystemParam<Self::ArgsMut>,
        mut all_task_queues: Query<&mut TaskQueue>,
        mut commands: Commands,
        mut all_task_started_event_writers: AllTaskStartedEventWriters,
    ) -> BevyResult
    where
        TaskData: ShipTaskData
            + TaskCreationEventHandler<'w, 's, TaskData, Args = Self::Args, ArgsMut = Self::ArgsMut>,
    {
        for event in events.read() {
            let mut task_queue = all_task_queues.get_mut(event.entity)?;
            match TaskData::create_tasks_for_command(
                event,
                &task_queue,
                &general_pathfinding_args,
                &args,
                &mut args_mut,
            ) {
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
                        &mut task_queue, // Since creation didn't fail, this shouldn't fail either.
                        &mut all_task_started_event_writers,
                        commands.reborrow(),
                    );
                }
            }
        }

        Ok(())
    }
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
    OwnEntityNotFound,
    TargetNotFound,
    BothNotFound,
    UnspecifiedError,
}

/// Applies the provided list of Tasks to the provided TaskQueue.
/// Should be called at the end of CreateTaskCommand Listeners.
fn apply_tasks(
    mut new_tasks: VecDeque<TaskKind>,
    task_insertion_mode: TaskInsertionMode,
    entity: Entity,
    queue: &mut TaskQueue,
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
            queue,
            entity.into(),
            &mut commands.entity(entity),
            all_task_started_event_writers,
        )
    }
}

/// A collection of system parameters necessary to do all generalized precondition checks and pathfinding
#[derive(SystemParam)]
pub(crate) struct GeneralPathfindingArgs<'w, 's> {
    pub relevant_entities: Query<'w, 's, (&'static InSector, &'static Transform)>,
    pub is_docked: Query<'w, 's, &'static IsDocked>,
    pub all_sectors: Query<'w, 's, &'static Sector>,
    pub all_transforms: Query<'w, 's, &'static SimulationTransform>,
}

use crate::can_task_be_cancelled_while_active;
use crate::ship_ai::create_tasks_following_path::create_tasks_to_follow_path;
use crate::ship_ai::tasks::apply_next_task;
use bevy::ecs::system::SystemParam;
use bevy::prelude::{BevyError, Commands, Entity, EventReader, Mut, Query, warn};
use common::components::task_kind::TaskKind;
use common::components::task_queue::TaskQueue;
use common::components::{InSector, Sector};
use common::events::task_events::{
    AllTaskStartedEventWriters, InsertTaskIntoQueueCommand, TaskInsertionMode,
};
use common::simulation_transform::SimulationTransform;
use common::types::ship_tasks::MoveToPosition;
use std::collections::VecDeque;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(SystemParam)]
pub(crate) struct MoveToPositionArgs<'w, 's> {
    pub ships: Query<'w, 's, (&'static mut TaskQueue, &'static InSector)>,
    pub all_sectors: Query<'w, 's, &'static Sector>,
    pub all_transforms: Query<'w, 's, &'static SimulationTransform>,
    pub commands: Commands<'w, 's>,
    pub all_task_started_event_writers: AllTaskStartedEventWriters<'w>,
}

#[derive(Debug)]
struct TaskCreationError {
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
enum TaskCreationErrorReason {
    ShipNotFound,
}

pub(crate) fn move_to_position_command_listener(
    mut events: EventReader<InsertTaskIntoQueueCommand<MoveToPosition>>,
    mut args: MoveToPositionArgs,
) {
    for event in events.read() {
        if let Err(e) = handle_event(event, &mut args) {
            warn!(
                "Error whilst running move_to_position_command_listener: {:?}",
                e
            );
        }
    }
}

fn handle_event(
    event: &InsertTaskIntoQueueCommand<MoveToPosition>,
    args: &mut MoveToPositionArgs,
) -> Result<(), BevyError> {
    let Ok((mut queue, in_sector)) = args.ships.get_mut(event.entity) else {
        return Err(TaskCreationError {
            entity: event.entity,
            reason: TaskCreationErrorReason::ShipNotFound,
        }
        .into());
    };

    let new_tasks = create_tasks(event, &args.all_sectors, &args.all_transforms, in_sector)?;

    apply_tasks(
        new_tasks,
        event.insertion_mode,
        event.entity,
        &mut queue,
        &mut args.all_task_started_event_writers,
        &mut args.commands,
    );

    Ok(())
}

fn create_tasks(
    event: &InsertTaskIntoQueueCommand<MoveToPosition>,
    all_sectors: &Query<&Sector>,
    all_transforms: &Query<&SimulationTransform>,
    in_sector: &InSector,
) -> Result<VecDeque<TaskKind>, BevyError> {
    let mut new_tasks = VecDeque::default();

    let target_position = event.task_data.sector_position;
    if target_position.sector != in_sector.sector {
        let path = pathfinding::find_path(
            all_sectors,
            all_transforms,
            in_sector.sector,
            all_transforms.get(event.entity)?.translation,
            target_position.sector,
            Some(target_position.local_position),
        )
        .unwrap();

        create_tasks_to_follow_path(&mut new_tasks, path);
    }

    new_tasks.push_back(TaskKind::MoveToPosition {
        data: event.task_data.clone(),
    });
    Ok(new_tasks)
}

/// Applies the provided list of Tasks to the provided TaskQueue.
// TODO: Every task creation should use this method
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

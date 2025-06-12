use crate::can_task_be_cancelled_while_active;
use crate::ship_ai::create_tasks_following_path::create_tasks_to_follow_path;
use crate::ship_ai::tasks::apply_next_task;
use bevy::ecs::error::panic;
use bevy::ecs::system::{StaticSystemParam, SystemParam};
use bevy::log::warn;
use bevy::math::Vec2;
use bevy::prelude::{BevyError, Commands, Entity, EventReader, Mut, Query, Transform};
use common::components::task_kind::TaskKind;
use common::components::task_queue::TaskQueue;
use common::components::{InSector, IsDocked, Sector};
use common::constants;
use common::constants::BevyResult;
use common::events::task_events::{
    AllTaskStartedEventWriters, InsertTaskIntoQueueCommand, TaskInsertionMode,
};
use common::simulation_transform::SimulationTransform;
use common::types::entity_wrappers::{SectorEntity, TypedEntity};
use common::types::ship_tasks;
use common::types::ship_tasks::{ShipTaskData, Undock};
use std::collections::VecDeque;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub(crate) trait TaskCreation<TaskData: ShipTaskData + 'static, Args: SystemParam> {
    /// Creates a VecDequeue with all subtasks necessary to achieve this Tasks.
    fn create_tasks_for_command(
        event: &InsertTaskIntoQueueCommand<TaskData>,
        task_queue: &TaskQueue,
        general_pathfinding_args: &GeneralPathfindingArgs,
        args: &mut StaticSystemParam<Args>,
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
    OwnEntityNotFound,
    TargetNotFound,
    BothNotFound,
    UnspecifiedError,
}

pub(crate) fn create_task_command_listener<TaskData, Args>(
    mut events: EventReader<InsertTaskIntoQueueCommand<TaskData>>,
    general_pathfinding_args: GeneralPathfindingArgs,
    mut args_for_creation: StaticSystemParam<Args>,
    mut all_task_queues: Query<&mut TaskQueue>,
    mut commands: Commands,
    mut all_task_started_event_writers: AllTaskStartedEventWriters,
) -> BevyResult
where
    TaskData: ShipTaskData + TaskCreation<TaskData, Args> + 'static,
    Args: SystemParam,
{
    for event in events.read() {
        let mut task_queue = all_task_queues.get_mut(event.entity)?;
        match TaskData::create_tasks_for_command(
            event,
            &task_queue,
            &general_pathfinding_args,
            &mut args_for_creation,
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

/// Applies the provided list of Tasks to the provided TaskQueue. Should be called at the end of CreateTaskCommand Listeners.
fn apply_tasks(
    mut new_tasks: VecDeque<TaskKind>,
    task_insertion_mode: TaskInsertionMode,
    entity: Entity,
    mut queue: &mut TaskQueue,
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

/// A collection of system parameters necessary to do all generalized precondition checks and pathfinding
#[derive(SystemParam)]
pub(crate) struct GeneralPathfindingArgs<'w, 's> {
    pub relevant_entities: Query<'w, 's, (&'static InSector, &'static Transform)>,
    pub is_docked: Query<'w, 's, &'static IsDocked>,
    pub all_sectors: Query<'w, 's, &'static Sector>,
    pub all_transforms: Query<'w, 's, &'static SimulationTransform>,
}

struct SectorAndDockingStatus {
    docked_at: Option<TypedEntity>,
    sector: SectorEntity,
}

fn get_sector(
    entity: Entity,
    in_sector_query: &Query<(&InSector, &Transform)>,
) -> Result<SectorEntity, BevyError> {
    Ok(in_sector_query.get(entity)?.0.sector)
}

/// Returns the target sector and position for the provided task.
fn get_task_end_sector_and_position(
    in_sector_query: &Query<(&InSector, &Transform)>,
    relevant_task: &TaskKind,
) -> Result<SectorAndDockingStatus, BevyError> {
    let result = match relevant_task {
        TaskKind::AwaitingSignal { .. } => {
            todo!(
                "This should never be the last task in a queue... actually, it might once players can send signals"
            )
        }
        TaskKind::Construct { data } => SectorAndDockingStatus {
            docked_at: None,
            sector: get_sector(data.target.into(), in_sector_query)?,
        },
        TaskKind::RequestAccess { .. } => {
            todo!("This should never be the last task in a queue")
        }
        TaskKind::DockAtEntity { data } => SectorAndDockingStatus {
            docked_at: Some(data.target),
            sector: get_sector(data.target.into(), in_sector_query)?,
        },
        TaskKind::Undock { data } => SectorAndDockingStatus {
            docked_at: None,
            sector: get_sector(data.from.into(), in_sector_query)?,
        },
        TaskKind::ExchangeWares { data } => {
            SectorAndDockingStatus {
                docked_at: Some(data.target), // TODO: Both are highly dynamic once target can be a ship
                sector: get_sector(data.target.into(), in_sector_query)?,
            }
        }
        TaskKind::MoveToEntity { data } => {
            SectorAndDockingStatus {
                docked_at: None, // TODO: Both are highly dynamic if target is a ship
                sector: get_sector(data.target.into(), in_sector_query)?,
            }
        }
        TaskKind::MoveToPosition { data } => SectorAndDockingStatus {
            docked_at: None,
            sector: get_sector(data.sector_position.sector.into(), in_sector_query)?,
        },
        TaskKind::UseGate { data } => SectorAndDockingStatus {
            docked_at: None,
            sector: get_sector(data.exit_sector.into(), in_sector_query)?,
        },
        TaskKind::MineAsteroid { data } => SectorAndDockingStatus {
            docked_at: None,
            sector: get_sector(data.target.into(), in_sector_query)?,
        },
        TaskKind::HarvestGas { data } => SectorAndDockingStatus {
            docked_at: None,
            sector: get_sector(data.target.into(), in_sector_query)?,
        },
    };

    Ok(result)
}

/// Creates all the necessary precondition (undock etc.) + movement tasks to move to a specific entity.
///
/// # Returns
/// A VecDequeue with the tasks.
pub fn create_preconditions_and_move_to_entity(
    entity: Entity,
    target_entity: TypedEntity,
    task_queue: &TaskQueue,
    args: &GeneralPathfindingArgs,
) -> Result<VecDeque<TaskKind>, BevyError> {
    let Ok((target_in_sector, _)) = args.relevant_entities.get(target_entity.into()) else {
        return Err(TaskCreationError {
            entity,
            reason: TaskCreationErrorReason::TargetNotFound,
        }
        .into());
    };

    let mut new_tasks = create_preconditions_and_move_to_sector(
        entity,
        task_queue,
        target_in_sector.sector,
        None, // TODO: Once the underlying logic uses local space
        args,
    )?;

    // TODO: MoveToEntity needs it's own subtask if it's not stationary, so we can listen for sector changes
    new_tasks.push_back(TaskKind::MoveToEntity {
        data: ship_tasks::MoveToEntity {
            target: target_entity,
            stop_at_target: true,
            desired_distance_to_target: constants::DOCKING_DISTANCE_TO_STATION,
        },
    });

    Ok(new_tasks)
}

/// Creates all the necessary precondition (undock etc.) + movement tasks to move to a specific sector.
///
/// # Returns
/// a VecDequeue with the tasks.
///
/// # Remarks
/// - If target_position is None, the first path that's found will be used - though it might not be the fastest path to the far end of the sector.
/// - If target_position is Some, this method won't add an extra MoveTo to said position, but will look for faster routes through other gates
/// - ***Transforms are currently world space, though ideally they'll eventually shift to SectorSpace***
pub fn create_preconditions_and_move_to_sector(
    entity: Entity,
    task_queue: &TaskQueue,
    target_sector: SectorEntity,
    target_position: Option<Vec2>, // TODO: Right now, this is the in global space!
    args: &GeneralPathfindingArgs,
) -> Result<VecDeque<TaskKind>, BevyError> {
    let sector_and_docking_status: SectorAndDockingStatus =
        if let Some(last_task) = task_queue.queue.back() {
            get_task_end_sector_and_position(&args.relevant_entities, last_task)?
        } else if let Some(active_task) = &task_queue.active_task {
            get_task_end_sector_and_position(&args.relevant_entities, active_task)?
        } else {
            let Ok((this_sector, _)) = args.relevant_entities.get(entity) else {
                return Err(TaskCreationError {
                    entity,
                    reason: TaskCreationErrorReason::OwnEntityNotFound,
                }
                .into());
            };

            SectorAndDockingStatus {
                sector: this_sector.sector,
                docked_at: args.is_docked.get(entity).map(|x| x.at).ok(),
            }
        };

    let mut new_tasks = VecDeque::default();

    // TODO: Either always add undock and skip it if we aren't docked,
    //       OR check if we are docked as a precondition in MoveTo[X] Commands.
    //      Probably better than checking it here.
    if let Some(docked_at) = sector_and_docking_status.docked_at {
        new_tasks.push_back(TaskKind::Undock {
            data: Undock {
                start_position: None,
                from: docked_at,
            },
        })
    }

    // TODO: We also need to kick out docked idle ships in case something else *needs* to dock for another task

    create_move_to_sector_tasks(
        entity,
        sector_and_docking_status.sector,
        target_sector,
        target_position,
        &args.all_sectors,
        &args.all_transforms,
        &mut new_tasks,
    )?;

    Ok(new_tasks)
}

/// Creates all the necessary tasks to move to a specific sector and adds them to the provided VecDequeue.
/// If target_position is None, the first path that's found will be used - though it might not be the fastest path to the far end of the sector.
/// If target_position is Some, this method won't add an extra MoveTo to said position, but will look for faster routes through other gates
/// Transforms are currently world space, though ideally they'll eventually shift to SectorSpace
fn create_move_to_sector_tasks(
    entity: Entity,
    current_sector: SectorEntity,
    target_sector: SectorEntity,
    target_position: Option<Vec2>,
    all_sectors: &Query<&Sector>,
    all_transforms: &Query<&SimulationTransform>,
    tasks: &mut VecDeque<TaskKind>,
) -> Result<(), BevyError> {
    if target_sector != current_sector {
        let path = pathfinding::find_path(
            all_sectors,
            all_transforms,
            current_sector,
            all_transforms.get(entity)?.translation,
            target_sector,
            target_position,
        )
        .unwrap();

        create_tasks_to_follow_path(tasks, path);
    }

    Ok(())
}

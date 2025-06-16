use crate::task_lifecycle_traits::task_creation::{
    GeneralPathfindingArgs, TaskCreationError, TaskCreationErrorReason,
};
use bevy::math::Vec2;
use bevy::prelude::{BevyError, Entity, Query, Transform};
use common::components::task_kind::TaskKind;
use common::components::task_queue::TaskQueue;
use common::components::{InSector, Sector};
use common::constants;
use common::simulation_transform::SimulationTransform;
use common::types::entity_wrappers::{SectorEntity, TypedEntity};
use common::types::ship_tasks;
use pathfinding::PathElement;
use std::collections::VecDeque;

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
            sector: data.sector_position.sector,
        },
        TaskKind::MoveToSector { data } => SectorAndDockingStatus {
            docked_at: None,
            sector: data.sector,
        },
        TaskKind::UseGate { data } => SectorAndDockingStatus {
            docked_at: None,
            sector: data.exit_sector,
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

/// Creates all the necessary precondition (undock etc.) + movement tasks to dock at a specific entity.
///
/// # Returns
/// A VecDequeue with the tasks.
pub fn create_preconditions_and_dock_at_entity(
    entity: Entity,
    target_entity: TypedEntity,
    task_queue: &TaskQueue,
    args: &GeneralPathfindingArgs,
) -> Result<VecDeque<TaskKind>, BevyError> {
    if let Ok(is_docked) = args.is_docked.get(entity) {
        if is_docked.at == target_entity {
            // Nothing to do, yay!
            return Ok(VecDeque::new());
        }
    }

    let mut new_tasks =
        create_preconditions_and_move_to_entity(entity, target_entity, task_queue, args)?;

    new_tasks.push_back(TaskKind::RequestAccess {
        data: ship_tasks::RequestAccess {
            target: target_entity,
            goal: ship_tasks::RequestAccessGoal::Docking,
        },
    });
    new_tasks.push_back(TaskKind::DockAtEntity {
        data: ship_tasks::DockAtEntity {
            target: target_entity,
        },
    });

    Ok(new_tasks)
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
        new_tasks.push_back(TaskKind::RequestAccess {
            data: ship_tasks::RequestAccess {
                target: docked_at,
                goal: ship_tasks::RequestAccessGoal::Undocking,
            },
        });
        new_tasks.push_back(TaskKind::Undock {
            data: ship_tasks::Undock {
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

/// Creates the individual tasks required to follow a precalculated path.
pub fn create_tasks_to_follow_path(queue: &mut VecDeque<TaskKind>, path: Vec<PathElement>) {
    for x in path {
        queue.push_back(TaskKind::MoveToEntity {
            data: ship_tasks::MoveToEntity {
                target: x.gate_pair.from.into(),
                stop_at_target: false,
                desired_distance_to_target: 0.0,
            },
        });
        queue.push_back(TaskKind::UseGate {
            data: ship_tasks::UseGate::new(x.gate_pair.from, x.exit_sector),
        })
    }
}

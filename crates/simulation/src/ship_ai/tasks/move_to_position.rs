use crate::ship_ai::TaskComponent;
use crate::ship_ai::create_tasks_following_path::create_tasks_to_follow_path;
use crate::ship_ai::ship_task::ShipTask;
use crate::ship_ai::task_creation::{TaskCreation, TaskCreationError, TaskCreationErrorReason};
use crate::ship_ai::task_result::TaskResult;
use crate::ship_ai::tasks::{move_to_entity, send_completion_events};
use bevy::ecs::system::{StaticSystemParam, SystemParam};
use bevy::prelude::{BevyError, Entity, EventWriter, Query, Res, Time};
use common::components::ship_velocity::ShipVelocity;
use common::components::task_kind::TaskKind;
use common::components::{Engine, InSector, Sector};
use common::events::task_events::{InsertTaskIntoQueueCommand, TaskCompletedEvent};
use common::simulation_transform::SimulationTransform;
use common::types::ship_tasks::MoveToPosition;
use std::collections::VecDeque;
use std::ops::Deref;
use std::sync::{Arc, Mutex};

impl TaskComponent for ShipTask<MoveToPosition> {
    fn can_be_cancelled_while_active() -> bool {
        true
    }
}

impl ShipTask<MoveToPosition> {
    pub fn run_tasks(
        event_writer: EventWriter<TaskCompletedEvent<MoveToPosition>>,
        time: Res<Time>,
        mut ships: Query<(Entity, &Self, &Engine, &mut ShipVelocity)>,
        all_transforms: Query<&SimulationTransform>,
    ) {
        let task_completions =
            Arc::new(Mutex::new(Vec::<TaskCompletedEvent<MoveToPosition>>::new()));
        let delta_seconds = time.delta_secs();

        ships
            .par_iter_mut()
            .for_each(
                |(entity, task, engine, mut velocity)| match move_to_entity::move_to_position(
                    entity,
                    task.global_position,
                    0.0,
                    true,
                    &all_transforms,
                    engine,
                    &mut velocity,
                    delta_seconds,
                ) {
                    TaskResult::Ongoing => {}
                    TaskResult::Finished | TaskResult::Aborted => task_completions
                        .lock()
                        .unwrap()
                        .push(TaskCompletedEvent::<MoveToPosition>::new(entity.into())),
                },
            );

        send_completion_events(event_writer, task_completions);
    }

    pub(crate) fn cancel_task_inside_queue() {
        // Nothing needs to be done
    }

    pub(crate) fn abort_running_task() {
        // Nothing needs to be done
    }
}

#[derive(SystemParam)]
pub(crate) struct MoveToPositionArgs<'w, 's> {
    pub ships: Query<'w, 's, &'static InSector>,
    pub all_sectors: Query<'w, 's, &'static Sector>,
    pub all_transforms: Query<'w, 's, &'static SimulationTransform>,
}

impl TaskCreation<MoveToPosition, MoveToPositionArgs<'_, '_>> for MoveToPosition {
    fn create_tasks_for_command(
        event: &InsertTaskIntoQueueCommand<MoveToPosition>,
        args: &StaticSystemParam<MoveToPositionArgs>,
    ) -> Result<VecDeque<TaskKind>, BevyError> {
        let args = args.deref();
        let Ok(in_sector) = args.ships.get(event.entity) else {
            return Err(TaskCreationError {
                entity: event.entity,
                reason: TaskCreationErrorReason::ShipNotFound,
            }
            .into());
        };

        let mut new_tasks = VecDeque::default();

        let target_position = event.task_data.sector_position;
        if target_position.sector != in_sector.sector {
            let path = pathfinding::find_path(
                &args.all_sectors,
                &args.all_transforms,
                in_sector.sector,
                args.all_transforms.get(event.entity)?.translation,
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
}

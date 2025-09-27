use crate::task_lifecycle_traits::task_cancellation_active::TaskCancellationForActiveTaskEventHandler;
use crate::task_lifecycle_traits::task_cancellation_in_queue::TaskCancellationForTaskInQueueEventHandler;
use crate::task_lifecycle_traits::task_completed::TaskCompletedEventHandler;
use crate::task_lifecycle_traits::task_creation::{
    GeneralPathfindingArgs, TaskCreationEventHandler,
};
use crate::task_lifecycle_traits::task_started::TaskStartedEventHandler;
use crate::task_lifecycle_traits::task_update_runner::TaskUpdateRunner;
use crate::task_metadata::TaskMetaData;
use crate::tasks::move_to_entity;
use crate::utility::ship_task::ShipTask;
use crate::utility::task_preconditions::create_preconditions_and_move_to_sector;
use crate::utility::task_result::TaskResult;
use bevy::ecs::system::{StaticSystemParam, SystemParam};
use bevy::math::Vec2;
use bevy::prelude::{BevyError, Entity, Query, Res, Time};
use common::components::Engine;
use common::components::ship_velocity::ShipVelocity;
use common::components::task_kind::TaskKind;
use common::components::task_queue::TaskQueue;
use common::events::task_events::{InsertTaskIntoQueueCommand, TaskCompletedEvent};
use common::simulation_transform::SimulationTransform;
use common::types::ship_tasks::MoveToPosition;
use std::collections::VecDeque;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};

#[derive(SystemParam)]
pub struct TaskUpdateRunnerArgs<'w, 's> {
    time: Res<'w, Time>,
    all_transforms: Query<'w, 's, &'static SimulationTransform>,
}

#[derive(SystemParam)]
pub struct TaskUpdateRunnerArgsMut<'w, 's> {
    ships: Query<
        'w,
        's,
        (
            Entity,
            &'static ShipTask<MoveToPosition>,
            &'static Engine,
            &'static mut ShipVelocity,
        ),
    >,
}

impl<'w, 's> TaskUpdateRunner<'w, 's, Self> for MoveToPosition {
    type Args = TaskUpdateRunnerArgs<'w, 's>;
    type ArgsMut = TaskUpdateRunnerArgsMut<'w, 's>;

    fn run_all_tasks(
        args: StaticSystemParam<Self::Args>,
        mut args_mut: StaticSystemParam<Self::ArgsMut>,
    ) -> Result<Arc<Mutex<Vec<TaskCompletedEvent<MoveToPosition>>>>, BevyError> {
        let args = args.deref();
        let args_mut = args_mut.deref_mut();

        let task_completions =
            Arc::new(Mutex::new(Vec::<TaskCompletedEvent<MoveToPosition>>::new()));
        let delta_seconds = args.time.delta_secs();

        args_mut
            .ships
            .par_iter_mut()
            .for_each(
                |(entity, task, engine, mut velocity)| match move_to_entity::move_to_position(
                    entity,
                    task.global_position,
                    0.0,
                    true,
                    &args.all_transforms,
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

        Ok(task_completions)
    }
}

impl<'w, 's> TaskCreationEventHandler<'w, 's, Self> for MoveToPosition {
    type Args = ();
    type ArgsMut = ();

    fn create_tasks_for_command(
        event: &InsertTaskIntoQueueCommand<MoveToPosition>,
        task_queue: &TaskQueue,
        general_pathfinding_args: &GeneralPathfindingArgs,
        _args: &StaticSystemParam<Self::Args>,
        _args_mut: &mut StaticSystemParam<Self::ArgsMut>,
    ) -> Result<VecDeque<TaskKind>, BevyError> {
        let mut new_tasks = create_preconditions_and_move_to_sector(
            event.entity,
            task_queue,
            event.task_data.sector_position.sector,
            None, // TODO: Once the underlying logic uses local space
            general_pathfinding_args,
        )?;

        new_tasks.push_back(TaskKind::MoveToPosition {
            data: event.task_data.clone(),
        });

        Ok(new_tasks)
    }
}

impl<'w, 's> TaskStartedEventHandler<'w, 's, Self> for MoveToPosition {
    type Args = ();
    type ArgsMut = ();

    fn skip_started() -> bool {
        true
    }
}

impl<'w, 's> TaskCancellationForTaskInQueueEventHandler<'w, 's, Self> for MoveToPosition {
    type Args = ();
    type ArgsMut = ();

    fn skip_cancelled_in_queue() -> bool {
        true
    }
}

impl<'w, 's> TaskCancellationForActiveTaskEventHandler<'w, 's, Self> for MoveToPosition {
    type Args = ();
    type ArgsMut = ();

    fn can_task_be_cancelled_while_active() -> bool {
        true
    }

    fn skip_cancelled_while_active() -> bool {
        true
    }
}

impl<'w, 's> TaskCompletedEventHandler<'w, 's, Self> for MoveToPosition {
    type Args = ();
    type ArgsMut = ();

    fn skip_completed() -> bool {
        true
    }
}

impl<'w, 's> TaskMetaData<'w, 's, Self> for MoveToPosition {
    fn task_target_position(&self, _all_transforms: &Query<&SimulationTransform>) -> Option<Vec2> {
        Some(self.global_position)
    }
}

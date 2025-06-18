use crate::TaskComponent;
use crate::task_lifecycle_traits::task_cancellation_active::TaskCancellationForActiveTaskEventHandler;
use crate::task_lifecycle_traits::task_cancellation_in_queue::TaskCancellationForTaskInQueueEventHandler;
use crate::task_lifecycle_traits::task_completed::TaskCompletedEventHandler;
use crate::task_lifecycle_traits::task_creation::{
    GeneralPathfindingArgs, TaskCreationEventHandler,
};
use crate::task_lifecycle_traits::task_started::TaskStartedEventHandler;
use crate::task_lifecycle_traits::task_update_runner::TaskUpdateRunner;
use crate::tasks::dock_at_entity;
use crate::utility::ship_task::ShipTask;
use crate::utility::task_result::TaskResult;
use bevy::ecs::system::{StaticSystemParam, SystemParam};
use bevy::prelude::{BevyError, Commands, Entity, EventWriter, Query, Res, Time, Visibility};
use common::components::ship_velocity::ShipVelocity;
use common::components::task_kind::TaskKind;
use common::components::task_queue::TaskQueue;
use common::components::{DockingBay, Engine, IsDocked};
use common::constants;
use common::events::send_signal_event::SendSignalEvent;
use common::events::task_events::TaskCompletedEvent;
use common::events::task_events::{InsertTaskIntoQueueCommand, TaskStartedEvent};
use common::simulation_transform::{SimulationScale, SimulationTransform};
use common::types::ship_tasks::Undock;
use std::collections::VecDeque;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};

impl TaskComponent for ShipTask<Undock> {
    fn can_be_cancelled_while_active() -> bool {
        false
    }
}

fn run(
    task: &ShipTask<Undock>,
    transform: &SimulationTransform,
    scale: &mut SimulationScale,
    velocity: &mut ShipVelocity,
    engine: &Engine,
    delta_seconds: f32,
) -> TaskResult {
    velocity.accelerate(engine, delta_seconds);
    if let Some(start_position) = task.start_position {
        let ratio = start_position.distance_squared(transform.translation)
            / constants::DOCKING_DISTANCE_TO_STATION_SQUARED;
        if ratio > 1.0 {
            scale.scale = 1.0;
            TaskResult::Finished
        } else {
            dock_at_entity::scale_based_on_docking_distance(scale, ratio);
            TaskResult::Ongoing
        }
    } else {
        // We just started and aren't even initialized yet
        TaskResult::Ongoing
    }
}

#[derive(SystemParam)]
pub struct TaskUpdateRunnerArgs<'w> {
    time: Res<'w, Time>,
}

#[derive(SystemParam)]
pub struct TaskUpdateRunnerArgsMut<'w, 's> {
    ships: Query<
        'w,
        's,
        (
            Entity,
            &'static ShipTask<Undock>,
            &'static SimulationTransform,
            &'static mut SimulationScale,
            &'static Engine,
            &'static mut ShipVelocity,
        ),
    >,
}

impl<'w, 's> TaskUpdateRunner<'w, 's, Self> for Undock {
    type Args = TaskUpdateRunnerArgs<'w>;
    type ArgsMut = TaskUpdateRunnerArgsMut<'w, 's>;

    fn run_all_tasks(
        args: StaticSystemParam<Self::Args>,
        mut args_mut: StaticSystemParam<Self::ArgsMut>,
    ) -> Result<Arc<Mutex<Vec<TaskCompletedEvent<Undock>>>>, BevyError> {
        let args = args.deref();
        let args_mut = args_mut.deref_mut();

        let task_completions = Arc::new(Mutex::new(Vec::<TaskCompletedEvent<Undock>>::new()));
        let delta_seconds = args.time.delta_secs();

        args_mut.ships.par_iter_mut().for_each(
            |(entity, task, transform, mut scale, engine, mut velocity)| match run(
                task,
                transform,
                &mut scale,
                &mut velocity,
                engine,
                delta_seconds,
            ) {
                TaskResult::Ongoing => {}
                TaskResult::Finished | TaskResult::Aborted => task_completions
                    .lock()
                    .unwrap()
                    .push(TaskCompletedEvent::<Undock>::new(entity.into())),
            },
        );

        Ok(task_completions)
    }
}

impl<'w, 's> TaskCreationEventHandler<'w, 's, Self> for Undock {
    type Args = ();
    type ArgsMut = ();

    fn create_tasks_for_command(
        _event: &InsertTaskIntoQueueCommand<Undock>,
        _task_queue: &TaskQueue,
        _general_pathfinding_args: &GeneralPathfindingArgs,
        _args: &StaticSystemParam<Self::Args>,
        _args_mut: &mut StaticSystemParam<Self::ArgsMut>,
    ) -> Result<VecDeque<TaskKind>, BevyError> {
        todo!()
    }
}

#[derive(SystemParam)]
pub struct TaskStartedArgsMut<'w, 's> {
    commands: Commands<'w, 's>,
    docking_bays: Query<'w, 's, &'static mut DockingBay>,
    all_ships_with_task: Query<
        'w,
        's,
        (
            Entity,
            &'static mut ShipTask<Undock>,
            &'static SimulationTransform,
            &'static mut Visibility,
        ),
    >,
}

impl<'w, 's> TaskStartedEventHandler<'w, 's, Self> for Undock {
    type Args = ();
    type ArgsMut = TaskStartedArgsMut<'w, 's>;

    fn on_task_started(
        event: &TaskStartedEvent<Undock>,
        _args: &StaticSystemParam<Self::Args>,
        args_mut: &mut StaticSystemParam<Self::ArgsMut>,
    ) -> Result<(), BevyError> {
        let args_mut = args_mut.deref_mut();

        let (entity, mut task, transform, mut visibility) =
            args_mut.all_ships_with_task.get_mut(event.entity.into())?;

        *visibility = Visibility::Inherited;
        task.start_position = Some(transform.translation);
        args_mut.commands.entity(entity).remove::<IsDocked>();
        args_mut
            .docking_bays
            .get_mut(task.from.into())?
            .start_undocking(entity.into());

        Ok(())
    }
}

impl<'w, 's> TaskCancellationForTaskInQueueEventHandler<'w, 's, Self> for Undock {
    type Args = ();
    type ArgsMut = ();

    fn can_task_be_cancelled_while_in_queue() -> bool {
        true
    }

    fn skip_cancelled_in_queue() -> bool {
        true
    }
}

impl<'w, 's> TaskCancellationForActiveTaskEventHandler<'w, 's, Self> for Undock {
    type Args = ();
    type ArgsMut = ();
}

#[derive(SystemParam)]
pub struct TaskRunnerArgs<'w, 's> {
    all_ships_with_task: Query<'w, 's, &'static ShipTask<Undock>>,
}
#[derive(SystemParam)]
pub struct TaskRunnerArgsMut<'w, 's> {
    send_signal_event_writer: EventWriter<'w, SendSignalEvent>,
    docking_bays: Query<'w, 's, &'static mut DockingBay>,
}

impl<'w, 's> TaskCompletedEventHandler<'w, 's, Self> for Undock {
    type Args = TaskRunnerArgs<'w, 's>;
    type ArgsMut = TaskRunnerArgsMut<'w, 's>;

    fn on_task_completed(
        event: &TaskCompletedEvent<Undock>,
        args: &StaticSystemParam<Self::Args>,
        args_mut: &mut StaticSystemParam<Self::ArgsMut>,
    ) -> Result<(), BevyError> {
        let args = args.deref();
        let args_mut = args_mut.deref_mut();

        let task = args.all_ships_with_task.get(event.entity.into())?;
        let mut docking_bay = args_mut.docking_bays.get_mut(task.from.into())?;
        docking_bay.finish_undocking(&event.entity, &mut args_mut.send_signal_event_writer);

        Ok(())
    }
}

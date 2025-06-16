use crate::TaskComponent;
use crate::task_lifecycle_traits::task_cancellation_active::TaskCancellationForActiveTaskEventHandler;
use crate::task_lifecycle_traits::task_cancellation_in_queue::TaskCancellationForTaskInQueueEventHandler;
use crate::task_lifecycle_traits::task_completed::TaskCompletedEventHandler;
use crate::task_lifecycle_traits::task_creation::{
    GeneralPathfindingArgs, TaskCreationEventHandler,
};
use crate::task_lifecycle_traits::task_started::TaskStartedEventHandler;
use crate::task_lifecycle_traits::task_update_runner::TaskUpdateRunner;
use crate::tasks::{move_to_entity, send_completion_events};
use crate::utility::ship_task::ShipTask;
use crate::utility::task_result::TaskResult;
use bevy::ecs::system::{StaticSystemParam, SystemParam};
use bevy::prelude::{
    BevyError, Commands, Entity, EventReader, EventWriter, FloatExt, Query, Res, Time, Visibility,
};
use common::components;
use common::components::ship_velocity::ShipVelocity;
use common::components::task_kind::TaskKind;
use common::components::task_queue::TaskQueue;
use common::components::{DockingBay, Engine};
use common::constants;
use common::constants::BevyResult;
use common::events::task_events::{
    InsertTaskIntoQueueCommand, TaskCanceledWhileInQueueEvent, TaskCompletedEvent, TaskStartedEvent,
};
use common::simulation_transform::{SimulationScale, SimulationTransform};
use common::types::ship_tasks::{AwaitingSignal, DockAtEntity};
use std::collections::VecDeque;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};

impl TaskComponent for ShipTask<DockAtEntity> {
    fn can_be_cancelled_while_active() -> bool {
        false
    }
}

pub fn scale_based_on_docking_distance(scale: &mut SimulationScale, ratio: f32) {
    if ratio < 0.5 {
        scale.scale = 1.0.lerp(0.0, (1.0 - ratio * 2.0).powi(3));
    } else {
        scale.scale = 1.0;
    }
}

fn scale_based_on_distance(
    task: &ShipTask<DockAtEntity>,
    this_entity: Entity,
    all_transforms: &Query<&SimulationTransform>,
    scale: &mut SimulationScale,
) {
    let [this_transform, target_transform] = all_transforms
        .get_many([this_entity, task.target.into()])
        .unwrap();

    let distance_squared = target_transform
        .translation
        .distance_squared(this_transform.translation);
    let ratio = distance_squared / constants::DOCKING_DISTANCE_TO_STATION_SQUARED;

    scale_based_on_docking_distance(scale, ratio);
}

impl<'w, 's> TaskCancellationForActiveTaskEventHandler<'w, 's, DockAtEntity> for DockAtEntity {
    type Args = ();
    type ArgsMut = ();

    // TODO: Technically, this can be cancelled: Just insert undock with inverted starting progress
}

impl<'w, 's> TaskCancellationForTaskInQueueEventHandler<'w, 's, DockAtEntity> for DockAtEntity {
    type Args = ();
    type ArgsMut = ();

    fn can_task_be_cancelled_while_in_queue() -> bool {
        true
    }

    fn cancellation_while_in_queue_event_listener(
        _events: EventReader<TaskCanceledWhileInQueueEvent<DockAtEntity>>,
        _args: StaticSystemParam<Self::Args>,
        _args_mut: StaticSystemParam<Self::ArgsMut>,
    ) -> BevyResult {
        Ok(())
    }
}

impl<'w, 's> TaskCreationEventHandler<'w, 's, DockAtEntity> for DockAtEntity {
    type Args = ();
    type ArgsMut = ();

    fn create_tasks_for_command(
        _event: &InsertTaskIntoQueueCommand<DockAtEntity>,
        _task_queue: &TaskQueue,
        _general_pathfinding_args: &GeneralPathfindingArgs,
        _args: &StaticSystemParam<Self::Args>,
        _args_mut: &mut StaticSystemParam<Self::ArgsMut>,
    ) -> Result<VecDeque<TaskKind>, BevyError> {
        todo!()
    }
}

impl<'w, 's> TaskStartedEventHandler<'w, 's, DockAtEntity> for DockAtEntity {
    type Args = ();
    type ArgsMut = ();

    fn on_task_started(
        _event: &TaskStartedEvent<DockAtEntity>,
        _args: &StaticSystemParam<Self::Args>,
        _args_mut: &mut StaticSystemParam<Self::ArgsMut>,
    ) -> Result<(), BevyError> {
        Ok(())
    }
}

#[derive(SystemParam)]
pub struct TaskRunnerArgs<'w, 's> {
    time: Res<'w, Time>,
    all_transforms: Query<'w, 's, &'static SimulationTransform>,
}

#[derive(SystemParam)]
pub struct TaskRunnerArgsMut<'w, 's> {
    ships: Query<
        'w,
        's,
        (
            Entity,
            &'static ShipTask<DockAtEntity>,
            &'static Engine,
            &'static mut ShipVelocity,
            &'static mut SimulationScale,
        ),
    >,
}

impl<'w, 's> TaskUpdateRunner<'w, 's, DockAtEntity> for DockAtEntity {
    type Args = TaskRunnerArgs<'w, 's>;
    type ArgsMut = TaskRunnerArgsMut<'w, 's>;

    fn run_all_tasks(
        event_writer: EventWriter<TaskCompletedEvent<DockAtEntity>>,
        args: StaticSystemParam<Self::Args>,
        mut args_mut: StaticSystemParam<Self::ArgsMut>,
    ) -> BevyResult {
        let args = args.deref();
        let args_mut = args_mut.deref_mut();

        let task_completions = Arc::new(Mutex::new(Vec::<TaskCompletedEvent<DockAtEntity>>::new()));

        args_mut.ships.par_iter_mut().for_each(
            |(entity, task, engine, mut velocity, mut scale)| match move_to_entity::move_to_entity(
                entity,
                task.target,
                0.0,
                true,
                &args.all_transforms,
                engine,
                &mut velocity,
                args.time.delta_secs(),
            ) {
                TaskResult::Ongoing => {
                    scale_based_on_distance(task, entity, &args.all_transforms, &mut scale);
                }
                TaskResult::Finished | TaskResult::Aborted => {
                    scale.scale = 0.0;

                    task_completions
                        .lock()
                        .unwrap()
                        .push(TaskCompletedEvent::<DockAtEntity>::new(entity.into()));
                }
            },
        );

        send_completion_events(event_writer, task_completions);

        Ok(())
    }
}

#[derive(SystemParam)]
pub struct TaskCompletedArgsMut<'w, 's> {
    commands: Commands<'w, 's>,
    all_ships_with_task: Query<'w, 's, (&'static mut Visibility, &'static ShipTask<DockAtEntity>)>,
    awaiting_signal_event_writer_for_next: EventWriter<'w, TaskCompletedEvent<AwaitingSignal>>,
    docking_bays: Query<'w, 's, &'static mut DockingBay>,
}

impl<'w, 's> TaskCompletedEventHandler<'w, 's, DockAtEntity> for DockAtEntity {
    type Args = ();
    type ArgsMut = TaskCompletedArgsMut<'w, 's>;

    fn on_task_completed(
        event: &TaskCompletedEvent<DockAtEntity>,
        _args: &StaticSystemParam<Self::Args>,
        args_mut: &mut StaticSystemParam<Self::ArgsMut>,
    ) -> Result<(), BevyError> {
        let args_mut = args_mut.deref_mut();

        let (mut visibility, task) = (args_mut.all_ships_with_task.get_mut(event.entity.into()))?;
        *visibility = Visibility::Hidden;

        let mut docking_bay = args_mut.docking_bays.get_mut(task.target.into())?;
        docking_bay.finish_docking(
            event.entity,
            &mut args_mut.awaiting_signal_event_writer_for_next,
        );

        let mut entity_commands = args_mut.commands.entity(event.entity.into());
        entity_commands.insert(components::IsDocked::new(task.target));

        Ok(())
    }
}

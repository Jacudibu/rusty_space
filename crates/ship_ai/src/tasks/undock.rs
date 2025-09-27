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
use bevy::math::Vec2;
use bevy::prelude::{BevyError, Commands, Entity, EventWriter, Query, Res, Rot2, Time, Visibility};
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
use std::f32::consts::FRAC_PI_2;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};

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
pub struct TaskStartedArgs<'w, 's> {
    all_task_queues: Query<'w, 's, &'static TaskQueue>,
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
            &'static mut Visibility,
        ),
    >,
    all_transforms: Query<'w, 's, &'static mut SimulationTransform>,
}

impl<'w, 's> TaskStartedEventHandler<'w, 's, Self> for Undock {
    type Args = TaskStartedArgs<'w, 's>;
    type ArgsMut = TaskStartedArgsMut<'w, 's>;

    fn on_task_started(
        event: &TaskStartedEvent<Undock>,
        args: &StaticSystemParam<Self::Args>,
        args_mut: &mut StaticSystemParam<Self::ArgsMut>,
    ) -> Result<(), BevyError> {
        let args_mut = args_mut.deref_mut();

        let (entity, mut task, mut visibility) =
            args_mut.all_ships_with_task.get_mut(event.entity.into())?;
        let task_queue = args.all_task_queues.get(entity)?;

        let undocking_origin_pos = args_mut.all_transforms.get(task.from.into())?.translation;

        let target_rotation = {
            if let Some(target_pos) = get_target_position_for_next_task_in_queue(
                task_queue,
                &args_mut.all_transforms.as_readonly(),
            ) {
                let delta_pos = target_pos - undocking_origin_pos;
                let rotation_in_radians = delta_pos.y.atan2(delta_pos.x);
                Rot2::radians(rotation_in_radians - FRAC_PI_2)
            } else {
                let rotation_in_radians = undocking_origin_pos.y.atan2(undocking_origin_pos.x);
                Rot2::radians(-rotation_in_radians + FRAC_PI_2)
            }
        };

        let mut entity_transform = args_mut.all_transforms.get_mut(entity)?;
        entity_transform.translation = undocking_origin_pos;
        entity_transform.rotation = target_rotation;

        *visibility = Visibility::Inherited;
        task.start_position = Some(entity_transform.translation);
        args_mut.commands.entity(entity).remove::<IsDocked>();
        args_mut
            .docking_bays
            .get_mut(task.from.into())?
            .start_undocking(entity.into());

        Ok(())
    }
}

/// Iterates through the provided [TaskQueue] in an attempt to find a target position for one of them.
fn get_target_position_for_next_task_in_queue(
    task_queue: &TaskQueue,
    all_transforms: &Query<&SimulationTransform>,
) -> Option<Vec2> {
    for x in &task_queue.queue {
        if let Some(target_pos) = determine_task_target_position(all_transforms, x) {
            return Some(target_pos);
        }
    }

    None
}

/// Determines the target position of the provided [TaskKind].
/// # Returns
/// - [None] if the task doesn't specify (or care) about a target position
/// - [Some] with a [Vec3] in case the task specifies a target position
// TODO: That should probably be an extension method implemented via traits
fn determine_task_target_position(
    all_transforms: &Query<&SimulationTransform>,
    task: &TaskKind,
) -> Option<Vec2> {
    match task {
        TaskKind::ExchangeWares { .. } => None,
        TaskKind::MoveToEntity { data } => {
            Some(all_transforms.get(data.target.into()).unwrap().translation)
        }
        TaskKind::MoveToPosition { data } => Some(data.global_position),
        TaskKind::MoveToSector { .. } => None,
        TaskKind::UseGate { data } => Some(
            all_transforms
                .get(data.enter_gate.into())
                .unwrap()
                .translation,
        ),
        TaskKind::MineAsteroid { data } => {
            Some(all_transforms.get(data.target.into()).unwrap().translation)
        }
        TaskKind::HarvestGas { data } => {
            Some(all_transforms.get(data.target.into()).unwrap().translation)
        }
        TaskKind::AwaitingSignal { .. } => None,
        TaskKind::RequestAccess { .. } => None,
        TaskKind::DockAtEntity { data } => {
            Some(all_transforms.get(data.target.into()).unwrap().translation)
        }
        TaskKind::Undock { .. } => None,
        TaskKind::Construct { data } => {
            // Task target might become invalid during the frame where construction is finished
            if let Ok(target_transform) = all_transforms.get(data.target.into()) {
                Some(target_transform.translation)
            } else {
                None
            }
        }
    }
}

impl<'w, 's> TaskCancellationForTaskInQueueEventHandler<'w, 's, Self> for Undock {
    type Args = ();
    type ArgsMut = ();

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

use crate::task_lifecycle_traits::task_cancellation_active::TaskCancellationForActiveTaskEventHandler;
use crate::task_lifecycle_traits::task_cancellation_in_queue::TaskCancellationForTaskInQueueEventHandler;
use crate::task_lifecycle_traits::task_completed::TaskCompletedEventHandler;
use crate::task_lifecycle_traits::task_creation::{
    GeneralPathfindingArgs, TaskCreationEventHandler,
};
use crate::task_lifecycle_traits::task_started::TaskStartedEventHandler;
use crate::task_lifecycle_traits::task_update_runner::TaskUpdateRunner;
use crate::task_metadata;
use crate::task_metadata::TaskMetaData;
use crate::utility::ship_task::ShipTask;
use crate::utility::task_result::TaskResult;
use bevy::ecs::system::{StaticSystemParam, SystemParam};
use bevy::math::{Rot2, Vec2};
use bevy::prelude::{BevyError, Entity, Query, Res, Time, warn};
use common::components::Engine;
use common::components::ship_velocity::ShipVelocity;
use common::components::task_kind::TaskKind;
use common::components::task_queue::TaskQueue;
use common::events::task_events::{InsertTaskIntoQueueCommand, TaskCompletedEvent};
use common::simulation_transform::SimulationTransform;
use common::types::entity_wrappers::TypedEntity;
use common::types::ship_tasks::MoveToEntity;
use std::collections::VecDeque;
use std::f32::consts::{FRAC_PI_2, PI};
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};

pub(crate) fn move_to_position(
    this_entity: Entity,
    target_position: Vec2,
    distance_to_target: f32,
    stop_at_target: bool,
    all_transforms: &Query<&SimulationTransform>,
    engine: &Engine,
    velocity: &mut ShipVelocity,
    delta_seconds: f32,
) -> TaskResult {
    let entity_transform = all_transforms.get(this_entity).unwrap();
    let delta = target_position - entity_transform.translation;
    let distance = delta.length() - distance_to_target;

    let required_rotation_to_face_target =
        calculate_required_rotation_to_face_target(entity_transform.rotation, &delta);

    turn_to_target(
        engine,
        velocity,
        delta_seconds,
        required_rotation_to_face_target,
    );

    if are_we_facing_the_target(required_rotation_to_face_target) {
        if stop_at_target {
            let distance_to_stop =
                (velocity.forward * velocity.forward) / (2.0 * engine.deceleration);

            let distance_travelled_this_frame = velocity.forward * delta_seconds;

            if distance - distance_travelled_this_frame > distance_to_stop {
                velocity.accelerate(engine, delta_seconds);
            } else {
                velocity.decelerate(engine, delta_seconds);
            }
        } else {
            velocity.accelerate(engine, delta_seconds);
        }
    } else {
        velocity.decelerate(engine, delta_seconds);
    }

    if distance < 10.0 {
        if stop_at_target {
            if velocity.forward < 0.3 {
                velocity.halt_all_movement();
                TaskResult::Finished
            } else {
                TaskResult::Ongoing
            }
        } else {
            TaskResult::Finished
        }
    } else {
        TaskResult::Ongoing
    }
}

pub(crate) fn move_to_entity(
    this_entity: Entity,
    target: TypedEntity,
    distance_to_target: f32,
    stop_at_target: bool,
    all_transforms: &Query<&SimulationTransform>,
    engine: &Engine,
    velocity: &mut ShipVelocity,
    delta_seconds: f32,
) -> TaskResult {
    let Ok(target_transform) = all_transforms.get(target.into()) else {
        warn!(
            "Didn't find target transform for {:?}, aborting MoveToEntity task.",
            target
        );
        return TaskResult::Aborted;
    };

    move_to_position(
        this_entity,
        target_transform.translation,
        distance_to_target,
        stop_at_target,
        all_transforms,
        engine,
        velocity,
        delta_seconds,
    )
}

pub(crate) fn turn_to_target(
    engine: &Engine,
    velocity: &mut ShipVelocity,
    delta_seconds: f32,
    angle_difference: f32,
) {
    if angle_difference - velocity.angular > 0.0 {
        velocity.turn_left(engine, delta_seconds);
    } else {
        velocity.turn_right(engine, delta_seconds);
    }
}

pub(crate) fn calculate_required_rotation_to_face_target(
    own_transform_rotation: Rot2,
    delta_position_to_target: &Vec2,
) -> f32 {
    let own_rotation = own_transform_rotation.as_radians() + FRAC_PI_2;
    let ideal_rotation = delta_position_to_target.y.atan2(delta_position_to_target.x);
    let missing_rotation = ideal_rotation - own_rotation;

    // Normalize missing rotation to [-pi, pi]
    if missing_rotation > PI {
        missing_rotation - 2.0 * PI
    } else if missing_rotation < -PI {
        missing_rotation + 2.0 * PI
    } else {
        missing_rotation
    }
}

pub(crate) fn are_we_facing_the_target(angle_difference: f32) -> bool {
    angle_difference.abs() < std::f32::consts::FRAC_PI_3
}

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
            &'static ShipTask<MoveToEntity>,
            &'static Engine,
            &'static mut ShipVelocity,
        ),
    >,
}

impl<'w, 's> TaskUpdateRunner<'w, 's, Self> for MoveToEntity {
    type Args = TaskUpdateRunnerArgs<'w, 's>;
    type ArgsMut = TaskUpdateRunnerArgsMut<'w, 's>;

    fn run_all_tasks(
        args: StaticSystemParam<Self::Args>,
        mut args_mut: StaticSystemParam<Self::ArgsMut>,
    ) -> Result<Arc<Mutex<Vec<TaskCompletedEvent<Self>>>>, BevyError> {
        let args = args.deref();
        let args_mut = args_mut.deref_mut();

        let task_completions = Arc::new(Mutex::new(Vec::<TaskCompletedEvent<Self>>::new()));
        let delta_seconds = args.time.delta_secs();

        args_mut
            .ships
            .par_iter_mut()
            .for_each(|(entity, task, engine, mut velocity)| {
                match move_to_entity(
                    entity,
                    task.target,
                    task.desired_distance_to_target,
                    task.stop_at_target,
                    &args.all_transforms,
                    engine,
                    &mut velocity,
                    delta_seconds,
                ) {
                    TaskResult::Ongoing => {}
                    TaskResult::Finished | TaskResult::Aborted => task_completions
                        .lock()
                        .unwrap()
                        .push(TaskCompletedEvent::<Self>::new(entity.into())),
                }
            });

        Ok(task_completions)
    }
}

impl<'w, 's> TaskCreationEventHandler<'w, 's, Self> for MoveToEntity {
    type Args = ();
    type ArgsMut = ();

    fn create_tasks_for_command(
        _event: &InsertTaskIntoQueueCommand<Self>,
        _task_queue: &TaskQueue,
        _general_pathfinding_args: &GeneralPathfindingArgs,
        _args: &StaticSystemParam<Self::Args>,
        _args_mut: &mut StaticSystemParam<Self::ArgsMut>,
    ) -> Result<VecDeque<TaskKind>, BevyError> {
        todo!(
            "that's gotta wait until we have all the entity position update listening figured out"
        )
    }
}

impl<'w, 's> TaskStartedEventHandler<'w, 's, Self> for MoveToEntity {
    type Args = ();
    type ArgsMut = ();

    fn skip_started() -> bool {
        true
    }
}

impl<'w, 's> TaskCancellationForTaskInQueueEventHandler<'w, 's, Self> for MoveToEntity {
    type Args = ();
    type ArgsMut = ();

    fn skip_cancelled_in_queue() -> bool {
        true
    }
}

impl<'w, 's> TaskCancellationForActiveTaskEventHandler<'w, 's, Self> for MoveToEntity {
    type Args = ();
    type ArgsMut = ();

    fn can_task_be_cancelled_while_active() -> bool {
        true
    }

    fn skip_cancelled_while_active() -> bool {
        true
    }
}

impl<'w, 's> TaskCompletedEventHandler<'w, 's, Self> for MoveToEntity {
    type Args = ();
    type ArgsMut = ();

    fn skip_completed() -> bool {
        true
    }
}

impl<'w, 's> TaskMetaData<'w, 's, Self> for MoveToEntity {
    fn task_target_position(&self, all_transforms: &Query<&SimulationTransform>) -> Option<Vec2> {
        task_metadata::get_entity_global_position(all_transforms, self.target.into())
    }
}

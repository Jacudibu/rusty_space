use crate::ship_ai::TaskComponent;
use crate::ship_ai::ship_task::ShipTask;
use crate::ship_ai::task_result::TaskResult;
use crate::ship_ai::tasks::send_completion_events;
use bevy::math::{Rot2, Vec2};
use bevy::prelude::{Entity, EventWriter, Query, Res, Time, warn};
use common::components::Engine;
use common::components::ship_velocity::ShipVelocity;
use common::events::task_events::TaskCompletedEvent;
use common::simulation_transform::SimulationTransform;
use common::types::entity_wrappers::TypedEntity;
use common::types::ship_tasks::MoveToEntity;
use std::f32::consts::{FRAC_PI_2, PI};
use std::sync::{Arc, Mutex};

impl TaskComponent for ShipTask<MoveToEntity> {
    fn can_be_cancelled_while_active() -> bool {
        true
    }
}

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

impl ShipTask<MoveToEntity> {
    pub fn run_tasks(
        event_writer: EventWriter<TaskCompletedEvent<MoveToEntity>>,
        time: Res<Time>,
        mut ships: Query<(Entity, &Self, &Engine, &mut ShipVelocity)>,
        all_transforms: Query<&SimulationTransform>,
    ) {
        let task_completions = Arc::new(Mutex::new(Vec::<TaskCompletedEvent<MoveToEntity>>::new()));
        let delta_seconds = time.delta_secs();

        ships
            .par_iter_mut()
            .for_each(|(entity, task, engine, mut velocity)| {
                match move_to_entity(
                    entity,
                    task.target,
                    task.desired_distance_to_target,
                    task.stop_at_target,
                    &all_transforms,
                    engine,
                    &mut velocity,
                    delta_seconds,
                ) {
                    TaskResult::Ongoing => {}
                    TaskResult::Finished | TaskResult::Aborted => task_completions
                        .lock()
                        .unwrap()
                        .push(TaskCompletedEvent::<MoveToEntity>::new(entity.into())),
                }
            });

        send_completion_events(event_writer, task_completions);
    }

    pub(crate) fn cancel_task_inside_queue() {
        // Nothing needs to be done
    }

    pub(crate) fn abort_running_task() {
        // Nothing needs to be done
    }
}

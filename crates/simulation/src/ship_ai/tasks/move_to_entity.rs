use crate::ship_ai::TaskComponent;
use crate::ship_ai::ship_task::ShipTask;
use crate::ship_ai::task_result::TaskResult;
use crate::ship_ai::tasks::send_completion_events;
use bevy::prelude::{Entity, EventWriter, Query, Res, Time, warn};
use common::components::Engine;
use common::components::ship_velocity::ShipVelocity;
use common::events::task_events::TaskCompletedEvent;
use common::simulation_transform::SimulationTransform;
use common::types::entity_wrappers::TypedEntity;
use common::types::ship_tasks::MoveToEntity;
use std::sync::{Arc, Mutex};

impl TaskComponent for ShipTask<MoveToEntity> {
    fn can_be_aborted() -> bool {
        true
    }
}

pub fn move_to_entity(
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
    let entity_transform = all_transforms.get(this_entity).unwrap();
    let delta = target_transform.translation - entity_transform.translation;

    let own_rotation = entity_transform.rotation.as_radians();
    let own_rotation = own_rotation + std::f32::consts::FRAC_PI_2;

    let target = delta.y.atan2(delta.x);
    let mut angle_difference = target - own_rotation;

    if angle_difference > std::f32::consts::PI {
        angle_difference -= 2.0 * std::f32::consts::PI;
    } else if angle_difference < -std::f32::consts::PI {
        angle_difference += 2.0 * std::f32::consts::PI;
    }

    if angle_difference - velocity.angular > 0.0 {
        velocity.turn_left(engine, delta_seconds);
    } else {
        velocity.turn_right(engine, delta_seconds);
    }

    let distance = delta.length() - distance_to_target;

    if angle_difference.abs() > std::f32::consts::FRAC_PI_3 {
        velocity.decelerate(engine, delta_seconds);
    } else if stop_at_target {
        let distance_to_stop = (velocity.forward * velocity.forward) / (2.0 * engine.deceleration);

        let distance_travelled_this_frame = velocity.forward * delta_seconds;

        if distance - distance_travelled_this_frame > distance_to_stop {
            velocity.accelerate(engine, delta_seconds);
        } else {
            velocity.decelerate(engine, delta_seconds);
        }
    } else {
        velocity.accelerate(engine, delta_seconds);
    }

    if distance < 10.0 {
        if stop_at_target {
            if velocity.forward < 0.3 {
                velocity.force_stop();
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
}

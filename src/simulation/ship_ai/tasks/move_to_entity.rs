use crate::simulation::physics::ShipVelocity;
use crate::simulation::prelude::TaskComponent;
use crate::simulation::ship_ai::task_events::TaskCompletedEvent;
use crate::simulation::ship_ai::task_result::TaskResult;
use crate::simulation::ship_ai::tasks::send_completion_events;
use crate::simulation::transform::simulation_transform::SimulationTransform;
use crate::utils::TypedEntity;
use bevy::prelude::{Component, Entity, EventWriter, Query, Res, Time, warn};
use common::components::Engine;
use std::sync::{Arc, Mutex};

/// Ships with this [TaskComponent] are currently moving towards another entity.
#[derive(Component)]
#[component(immutable)]
pub struct MoveToEntity {
    /// The entity to which we are moving.
    pub target: TypedEntity,

    /// Whether the ship should slow down as it reaches the target, or just zoom past it.
    pub stop_at_target: bool,

    /// In case that we stop at the target, how far from it would be the perfect distance to do so?
    /// 0 would be right on top.
    pub desired_distance_to_target: f32,
}
impl TaskComponent for MoveToEntity {
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

impl MoveToEntity {
    pub fn run_tasks(
        event_writer: EventWriter<TaskCompletedEvent<Self>>,
        time: Res<Time>,
        mut ships: Query<(Entity, &Self, &Engine, &mut ShipVelocity)>,
        all_transforms: Query<&SimulationTransform>,
    ) {
        let task_completions = Arc::new(Mutex::new(Vec::<TaskCompletedEvent<Self>>::new()));
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
                        .push(TaskCompletedEvent::<Self>::new(entity.into())),
                }
            });

        send_completion_events(event_writer, task_completions);
    }
}

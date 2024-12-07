use crate::components::Engine;
use crate::simulation::physics::ShipVelocity;
use crate::simulation::prelude::SimulationTime;
use crate::simulation::ship_ai::task_finished_event::TaskFinishedEvent;
use crate::simulation::ship_ai::task_queue::TaskQueue;
use crate::simulation::ship_ai::task_result::TaskResult;
use crate::simulation::ship_ai::tasks;
use crate::simulation::ship_ai::tasks::send_completion_events;
use crate::simulation::transform::simulation_transform::SimulationTransform;
use crate::utils::TypedEntity;
use bevy::log::error;
use bevy::prelude::{
    warn, Commands, Component, Entity, EventReader, EventWriter, Query, Res, Time, With,
};
use std::sync::{Arc, Mutex};

#[derive(Component)]
pub struct MoveToEntity {
    pub target: TypedEntity,
    pub stop_at_target: bool,
    pub distance_to_target: f32,
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
        event_writer: EventWriter<TaskFinishedEvent<Self>>,
        time: Res<Time>,
        mut ships: Query<(Entity, &Self, &Engine, &mut ShipVelocity)>,
        all_transforms: Query<&SimulationTransform>,
    ) {
        let task_completions = Arc::new(Mutex::new(Vec::<TaskFinishedEvent<Self>>::new()));
        let delta_seconds = time.delta_secs();

        ships
            .par_iter_mut()
            .for_each(|(entity, task, engine, mut velocity)| {
                match move_to_entity(
                    entity,
                    task.target,
                    task.distance_to_target,
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
                        .push(TaskFinishedEvent::<Self>::new(entity)),
                }
            });

        send_completion_events(event_writer, task_completions);
    }

    pub fn complete_tasks(
        mut commands: Commands,
        mut event_reader: EventReader<TaskFinishedEvent<Self>>,
        mut all_ships_with_task: Query<&mut TaskQueue, With<Self>>,
        simulation_time: Res<SimulationTime>,
    ) {
        let now = simulation_time.now();

        for event in event_reader.read() {
            if let Ok(mut queue) = all_ships_with_task.get_mut(event.entity) {
                tasks::remove_task_and_add_next_in_queue::<Self>(
                    &mut commands,
                    event.entity,
                    &mut queue,
                    now,
                );
            } else {
                error!(
                    "Unable to find entity for task completion: {}",
                    event.entity
                );
            }
        }
    }
}

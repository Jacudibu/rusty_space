use crate::components::{Engine, Velocity};
use crate::ship_ai::task_finished_event::TaskFinishedEvent;
use crate::ship_ai::task_queue::TaskQueue;
use crate::ship_ai::task_result::TaskResult;
use crate::ship_ai::tasks;
use crate::ship_ai::tasks::send_completion_events;
use bevy::log::error;
use bevy::prelude::{
    warn, Commands, Component, Entity, EulerRot, EventReader, EventWriter, Query, Res, Time,
    Transform, With,
};
use std::sync::{Arc, Mutex};

#[derive(Component)]
pub struct MoveToEntity {
    pub target: Entity,
}

impl MoveToEntity {
    pub fn run(
        &self,
        this_entity: Entity,
        all_transforms: &Query<&Transform>,
        engine: &Engine,
        velocity: &mut Velocity,
        time: &Time,
    ) -> TaskResult {
        let Ok(target_transform) = all_transforms.get(self.target) else {
            warn!(
                "Didn't find target transform for {}, aborting MoveToEntity task.",
                self.target
            );
            return TaskResult::Aborted;
        };
        let entity_transform = all_transforms.get(this_entity).unwrap();
        let delta =
            target_transform.translation.truncate() - entity_transform.translation.truncate();

        let (_, _, own_rotation) = entity_transform.rotation.to_euler(EulerRot::XYZ);
        let own_rotation = own_rotation + std::f32::consts::FRAC_PI_2;

        let target = delta.y.atan2(delta.x);
        let mut angle_difference = target - own_rotation;

        if angle_difference > std::f32::consts::PI {
            angle_difference -= 2.0 * std::f32::consts::PI;
        } else if angle_difference < -std::f32::consts::PI {
            angle_difference += 2.0 * std::f32::consts::PI;
        }

        if angle_difference - velocity.angular > 0.0 {
            velocity.turn_left(engine, time.delta_seconds());
        } else {
            velocity.turn_right(engine, time.delta_seconds());
        }

        let distance = delta.length();

        if angle_difference.abs() > std::f32::consts::FRAC_PI_3 {
            velocity.decelerate(engine, time.delta_seconds());
        } else {
            let distance_to_stop =
                (velocity.forward * velocity.forward) / (2.0 * engine.deceleration);

            if distance > distance_to_stop {
                velocity.accelerate(engine, time.delta_seconds());
            } else {
                velocity.decelerate(engine, time.delta_seconds());
            }
        }

        if distance < 5.0 {
            TaskResult::Finished
        } else {
            TaskResult::Ongoing
        }
    }

    pub fn run_tasks(
        event_writer: EventWriter<TaskFinishedEvent<Self>>,
        time: Res<Time>,
        mut ships: Query<(Entity, &Self, &Engine, &mut Velocity)>,
        all_transforms: Query<&Transform>,
    ) {
        let task_completions = Arc::new(Mutex::new(Vec::<TaskFinishedEvent<Self>>::new()));

        ships
            .par_iter_mut()
            .for_each(|(entity, task, engine, mut velocity)| {
                match task.run(entity, &all_transforms, engine, &mut velocity, &time) {
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
        mut all_ships_with_task: Query<(Entity, &mut TaskQueue), With<Self>>,
    ) {
        for event in event_reader.read() {
            if let Ok((entity, mut queue)) = all_ships_with_task.get_mut(event.entity) {
                tasks::remove_task_and_add_new_one::<Self>(&mut commands, entity, &mut queue);
            } else {
                error!(
                    "Unable to find entity for task completion: {}",
                    event.entity
                );
            }
        }
    }
}

use crate::components::{Engine, InteractionQueue, IsDocked};
use crate::constants;
use crate::simulation::physics::ShipVelocity;
use crate::simulation::prelude::SimulationTime;
use crate::simulation::ship_ai::task_finished_event::TaskFinishedEvent;
use crate::simulation::ship_ai::task_queue::TaskQueue;
use crate::simulation::ship_ai::task_result::TaskResult;
use crate::simulation::ship_ai::tasks::{finish_interaction, send_completion_events};
use crate::simulation::ship_ai::{tasks, AwaitingSignal};
use crate::simulation::transform::simulation_transform::SimulationTransform;
use bevy::log::error;
use bevy::prelude::{
    Commands, Component, Entity, EventReader, EventWriter, Query, Res, Time, Vec2, Visibility, With,
};
use std::sync::{Arc, Mutex};

#[derive(Component)]
pub struct Undock {
    start_position: Option<Vec2>,
}

impl Undock {
    pub fn new() -> Self {
        Self {
            start_position: None,
        }
    }

    fn run(
        &self,
        transform: &SimulationTransform,
        velocity: &mut ShipVelocity,
        engine: &Engine,
        delta_seconds: f32,
    ) -> TaskResult {
        velocity.accelerate(engine, delta_seconds);
        if let Some(start_position) = self.start_position {
            if start_position.distance_squared(transform.translation)
                > constants::DOCKING_DISTANCE_TO_STATION_SQUARED
            {
                TaskResult::Finished
            } else {
                TaskResult::Ongoing
            }
        } else {
            // We just started and aren't even initialized yet
            TaskResult::Ongoing
        }
    }

    pub fn run_tasks(
        event_writer: EventWriter<TaskFinishedEvent<Self>>,
        time: Res<Time>,
        mut ships: Query<(
            Entity,
            &Self,
            &SimulationTransform,
            &Engine,
            &mut ShipVelocity,
        )>,
    ) {
        let task_completions = Arc::new(Mutex::new(Vec::<TaskFinishedEvent<Self>>::new()));
        let delta_seconds = time.delta_seconds();

        ships
            .par_iter_mut()
            .for_each(|(entity, task, transform, engine, mut velocity)| {
                match task.run(transform, &mut velocity, engine, delta_seconds) {
                    TaskResult::Ongoing => {}
                    TaskResult::Finished | TaskResult::Aborted => task_completions
                        .lock()
                        .unwrap()
                        .push(TaskFinishedEvent::<Self>::new(entity)),
                }
            });

        send_completion_events(event_writer, task_completions);
    }

    #[allow(clippy::type_complexity)]
    pub fn on_task_creation(
        mut commands: Commands,
        mut all_ships_with_task: Query<(
            Entity,
            &mut Self,
            &SimulationTransform,
            &mut Visibility,
            &IsDocked,
        )>,
        mut interaction_queues: Query<&mut InteractionQueue>,
        mut signal_writer: EventWriter<TaskFinishedEvent<AwaitingSignal>>,
    ) {
        // Compared to the other task_creation thingies we can cheat a little since we got IsDocked as a useful marker
        for (entity, mut task, transform, mut visibility, is_docked) in
            all_ships_with_task.iter_mut()
        {
            finish_interaction(
                is_docked.at.into(),
                &mut interaction_queues,
                &mut signal_writer,
            );

            *visibility = Visibility::Inherited;
            task.start_position = Some(transform.translation);
            commands.entity(entity).remove::<IsDocked>();
        }
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

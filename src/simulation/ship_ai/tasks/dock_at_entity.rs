use crate::components;
use crate::components::Engine;
use crate::simulation::physics::ShipVelocity;
use crate::simulation::prelude::SimulationTime;
use crate::simulation::ship_ai::task_finished_event::TaskFinishedEvent;
use crate::simulation::ship_ai::task_queue::TaskQueue;
use crate::simulation::ship_ai::task_result::TaskResult;
use crate::simulation::ship_ai::tasks;
use crate::simulation::ship_ai::tasks::{move_to_entity, send_completion_events};
use crate::simulation::transform::simulation_transform::SimulationTransform;
use crate::utils::TypedEntity;
use bevy::log::error;
use bevy::prelude::{
    Commands, Component, Entity, EventReader, EventWriter, Query, Res, Time, Visibility, With,
};
use std::sync::{Arc, Mutex};

#[derive(Component)]
pub struct DockAtEntity {
    pub target: TypedEntity,
}

impl DockAtEntity {
    pub fn new(target: TypedEntity) -> Self {
        Self { target }
    }

    pub fn run_tasks(
        event_writer: EventWriter<TaskFinishedEvent<Self>>,
        time: Res<Time>,
        mut ships: Query<(Entity, &Self, &Engine, &mut ShipVelocity)>,
        all_transforms: Query<&SimulationTransform>,
    ) {
        let task_completions = Arc::new(Mutex::new(Vec::<TaskFinishedEvent<Self>>::new()));

        ships
            .par_iter_mut()
            .for_each(
                |(entity, task, engine, mut velocity)| match move_to_entity::move_to_entity(
                    entity,
                    task.target,
                    0.0,
                    true,
                    &all_transforms,
                    engine,
                    &mut velocity,
                    time.delta_seconds(),
                ) {
                    TaskResult::Ongoing => {}
                    TaskResult::Finished | TaskResult::Aborted => task_completions
                        .lock()
                        .unwrap()
                        .push(TaskFinishedEvent::<Self>::new(entity)),
                },
            );

        send_completion_events(event_writer, task_completions);
    }

    pub fn complete_tasks(
        mut commands: Commands,
        mut event_reader: EventReader<TaskFinishedEvent<Self>>,
        mut all_ships_with_task: Query<(&mut TaskQueue, &mut Visibility, &Self)>,
        simulation_time: Res<SimulationTime>,
    ) {
        let now = simulation_time.now();

        for event in event_reader.read() {
            if let Ok((mut queue, mut visibility, task)) = all_ships_with_task.get_mut(event.entity)
            {
                *visibility = Visibility::Hidden;

                let mut entity_commands = commands.entity(event.entity);
                entity_commands.insert(components::IsDocked::new(task.target));

                tasks::remove_task_and_add_next_in_queue_to_entity_commands::<Self>(
                    &mut entity_commands,
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

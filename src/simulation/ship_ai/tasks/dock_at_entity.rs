use crate::components::Engine;
use crate::simulation::physics::ShipVelocity;
use crate::simulation::prelude::SimulationTime;
use crate::simulation::ship_ai::task_finished_event::TaskFinishedEvent;
use crate::simulation::ship_ai::task_queue::TaskQueue;
use crate::simulation::ship_ai::task_result::TaskResult;
use crate::simulation::ship_ai::task_started_event::AllTaskStartedEventWriters;
use crate::simulation::ship_ai::tasks;
use crate::simulation::ship_ai::tasks::{move_to_entity, send_completion_events};
use crate::simulation::transform::simulation_transform::{SimulationScale, SimulationTransform};
use crate::utils::TypedEntity;
use crate::{components, constants};
use bevy::log::error;
use bevy::prelude::{
    Commands, Component, Entity, EventReader, EventWriter, FloatExt, Query, Res, Time, Visibility,
};
use std::sync::{Arc, Mutex};

#[derive(Component)]
pub struct DockAtEntity {
    pub target: TypedEntity,
}

pub fn scale_based_on_docking_distance(scale: &mut SimulationScale, ratio: f32) {
    if ratio < 0.5 {
        scale.scale = 1.0.lerp(0.0, (1.0 - ratio * 2.0).powi(3));
    } else {
        scale.scale = 1.0;
    }
}

impl DockAtEntity {
    pub fn new(target: TypedEntity) -> Self {
        Self { target }
    }

    fn scale_based_on_distance(
        &self,
        this_entity: Entity,
        all_transforms: &Query<&SimulationTransform>,
        scale: &mut SimulationScale,
    ) {
        let [this_transform, target_transform] = all_transforms
            .get_many([this_entity, self.target.into()])
            .unwrap();

        let distance_squared = target_transform
            .translation
            .distance_squared(this_transform.translation);
        let ratio = distance_squared / constants::DOCKING_DISTANCE_TO_STATION_SQUARED;

        scale_based_on_docking_distance(scale, ratio);
    }

    pub fn run_tasks(
        event_writer: EventWriter<TaskFinishedEvent<Self>>,
        time: Res<Time>,
        mut ships: Query<(
            Entity,
            &Self,
            &Engine,
            &mut ShipVelocity,
            &mut SimulationScale,
        )>,
        all_transforms: Query<&SimulationTransform>,
    ) {
        let task_completions = Arc::new(Mutex::new(Vec::<TaskFinishedEvent<Self>>::new()));

        ships
            .par_iter_mut()
            .for_each(|(entity, task, engine, mut velocity, mut scale)| {
                match move_to_entity::move_to_entity(
                    entity,
                    task.target,
                    0.0,
                    true,
                    &all_transforms,
                    engine,
                    &mut velocity,
                    time.delta_secs(),
                ) {
                    TaskResult::Ongoing => {
                        task.scale_based_on_distance(entity, &all_transforms, &mut scale);
                    }
                    TaskResult::Finished | TaskResult::Aborted => {
                        scale.scale = 0.0;

                        task_completions
                            .lock()
                            .unwrap()
                            .push(TaskFinishedEvent::<Self>::new(entity));
                    }
                }
            });

        send_completion_events(event_writer, task_completions);
    }

    pub fn complete_tasks(
        mut commands: Commands,
        mut event_reader: EventReader<TaskFinishedEvent<Self>>,
        mut all_ships_with_task: Query<(&mut TaskQueue, &mut Visibility, &Self)>,
        simulation_time: Res<SimulationTime>,
        mut task_started_event_writers: AllTaskStartedEventWriters,
    ) {
        let now = simulation_time.now();

        for event in event_reader.read() {
            if let Ok((mut queue, mut visibility, task)) = all_ships_with_task.get_mut(event.entity)
            {
                *visibility = Visibility::Hidden;

                let mut entity_commands = commands.entity(event.entity);
                entity_commands.insert(components::IsDocked::new(task.target));

                tasks::remove_task_and_add_next_in_queue_to_entity_commands::<Self>(
                    event.entity,
                    &mut entity_commands,
                    &mut queue,
                    now,
                    &mut task_started_event_writers,
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

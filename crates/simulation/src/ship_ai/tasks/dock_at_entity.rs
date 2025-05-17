use crate::ship_ai::TaskComponent;
use crate::ship_ai::ship_task::ShipTask;
use crate::ship_ai::task_result::TaskResult;
use crate::ship_ai::tasks::{move_to_entity, send_completion_events};
use bevy::log::error;
use bevy::prelude::{
    Commands, Entity, EventReader, EventWriter, FloatExt, Query, Res, Time, Visibility,
};
use common::components;
use common::components::Engine;
use common::components::ship_velocity::ShipVelocity;
use common::constants;
use common::events::task_events::TaskCompletedEvent;
use common::simulation_transform::{SimulationScale, SimulationTransform};
use common::types::ship_tasks::DockAtEntity;
use std::sync::{Arc, Mutex};

impl TaskComponent for ShipTask<DockAtEntity> {
    fn can_be_aborted() -> bool {
        true
    }
}

pub fn scale_based_on_docking_distance(scale: &mut SimulationScale, ratio: f32) {
    if ratio < 0.5 {
        scale.scale = 1.0.lerp(0.0, (1.0 - ratio * 2.0).powi(3));
    } else {
        scale.scale = 1.0;
    }
}

impl ShipTask<DockAtEntity> {
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
        event_writer: EventWriter<TaskCompletedEvent<DockAtEntity>>,
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
        let task_completions = Arc::new(Mutex::new(Vec::<TaskCompletedEvent<DockAtEntity>>::new()));

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
                            .push(TaskCompletedEvent::<DockAtEntity>::new(entity.into()));
                    }
                }
            });

        send_completion_events(event_writer, task_completions);
    }

    pub fn complete_tasks(
        mut commands: Commands,
        mut event_reader: EventReader<TaskCompletedEvent<DockAtEntity>>,
        mut all_ships_with_task: Query<(&mut Visibility, &Self)>,
    ) {
        for event in event_reader.read() {
            if let Ok((mut visibility, task)) = all_ships_with_task.get_mut(event.entity.into()) {
                *visibility = Visibility::Hidden;

                let mut entity_commands = commands.entity(event.entity.into());
                entity_commands.insert(components::IsDocked::new(task.target));
            } else {
                error!(
                    "Unable to find entity for DockAtEntity task completion: {}",
                    event.entity
                );
            }
        }
    }

    pub(crate) fn cancel_task_inside_queue() {
        // Nothing needs to be done.
    }
}

use crate::ship_ai::TaskComponent;
use crate::ship_ai::ship_task::ShipTask;
use crate::ship_ai::task_result::TaskResult;
use crate::ship_ai::tasks::{dock_at_entity, finish_interaction, send_completion_events};
use bevy::log::error;
use bevy::prelude::{Commands, Entity, EventReader, EventWriter, Query, Res, Time, Visibility};
use common::components::interaction_queue::InteractionQueue;
use common::components::ship_velocity::ShipVelocity;
use common::components::{Engine, IsDocked};
use common::constants;
use common::events::task_events::TaskCompletedEvent;
use common::events::task_events::TaskStartedEvent;
use common::simulation_transform::{SimulationScale, SimulationTransform};
use common::types::ship_tasks::{AwaitingSignal, Undock};
use std::sync::{Arc, Mutex};

impl TaskComponent for ShipTask<Undock> {
    fn can_be_aborted() -> bool {
        false
    }
}

impl ShipTask<Undock> {
    fn run(
        &self,
        transform: &SimulationTransform,
        scale: &mut SimulationScale,
        velocity: &mut ShipVelocity,
        engine: &Engine,
        delta_seconds: f32,
    ) -> TaskResult {
        velocity.accelerate(engine, delta_seconds);
        if let Some(start_position) = self.start_position {
            let ratio = start_position.distance_squared(transform.translation)
                / constants::DOCKING_DISTANCE_TO_STATION_SQUARED;
            if ratio > 1.0 {
                scale.scale = 1.0;
                TaskResult::Finished
            } else {
                dock_at_entity::scale_based_on_docking_distance(scale, ratio);
                TaskResult::Ongoing
            }
        } else {
            // We just started and aren't even initialized yet
            TaskResult::Ongoing
        }
    }

    pub fn run_tasks(
        event_writer: EventWriter<TaskCompletedEvent<Undock>>,
        time: Res<Time>,
        mut ships: Query<(
            Entity,
            &Self,
            &SimulationTransform,
            &mut SimulationScale,
            &Engine,
            &mut ShipVelocity,
        )>,
    ) {
        let task_completions = Arc::new(Mutex::new(Vec::<TaskCompletedEvent<Undock>>::new()));
        let delta_seconds = time.delta_secs();

        ships.par_iter_mut().for_each(
            |(entity, task, transform, mut scale, engine, mut velocity)| match task.run(
                transform,
                &mut scale,
                &mut velocity,
                engine,
                delta_seconds,
            ) {
                TaskResult::Ongoing => {}
                TaskResult::Finished | TaskResult::Aborted => task_completions
                    .lock()
                    .unwrap()
                    .push(TaskCompletedEvent::<Undock>::new(entity.into())),
            },
        );

        send_completion_events(event_writer, task_completions);
    }

    #[allow(clippy::type_complexity)]
    pub fn on_task_started(
        mut commands: Commands,
        mut all_ships_with_task: Query<(
            Entity,
            &mut Self,
            &SimulationTransform,
            &mut Visibility,
            &IsDocked,
        )>,
        mut started_tasks: EventReader<TaskStartedEvent<Undock>>,
        mut interaction_queues: Query<&mut InteractionQueue>,
        mut signal_writer: EventWriter<TaskCompletedEvent<AwaitingSignal>>,
    ) {
        // Compared to the other task_creation thingies we can cheat a little since we got IsDocked as a useful marker
        for task in started_tasks.read() {
            let Ok((entity, mut task, transform, mut visibility, is_docked)) =
                all_ships_with_task.get_mut(task.entity.into())
            else {
                error!(
                    "Was unable to start undock task for entity {:?}: Entity not found.",
                    task.entity
                );
                continue;
            };

            finish_interaction(
                is_docked.at.into(),
                &mut interaction_queues,
                &mut signal_writer,
            );

            *visibility = Visibility::Inherited;
            task.start_position = Some(transform.translation);
            //transform.scale = constants::DOCKING_SCALE_MIN;
            commands.entity(entity).remove::<IsDocked>();
        }
    }

    pub(crate) fn cancel_task_inside_queue() {
        // Nothing needs to be done
    }
}

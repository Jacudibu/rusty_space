use crate::ship_ai::TaskComponent;
use crate::ship_ai::ship_task::ShipTask;
use crate::ship_ai::task_result::TaskResult;
use crate::ship_ai::tasks::{move_to_entity, send_completion_events};
use bevy::prelude::{Entity, EventWriter, Query, Res, Time};
use common::components::Engine;
use common::components::ship_velocity::ShipVelocity;
use common::events::task_events::TaskCompletedEvent;
use common::simulation_transform::SimulationTransform;
use common::types::ship_tasks::MoveToPosition;
use std::sync::{Arc, Mutex};

impl TaskComponent for ShipTask<MoveToPosition> {
    fn can_be_cancelled_while_active() -> bool {
        true
    }
}

impl ShipTask<MoveToPosition> {
    pub fn run_tasks(
        event_writer: EventWriter<TaskCompletedEvent<MoveToPosition>>,
        time: Res<Time>,
        mut ships: Query<(Entity, &Self, &Engine, &mut ShipVelocity)>,
        all_transforms: Query<&SimulationTransform>,
    ) {
        let task_completions =
            Arc::new(Mutex::new(Vec::<TaskCompletedEvent<MoveToPosition>>::new()));
        let delta_seconds = time.delta_secs();

        ships
            .par_iter_mut()
            .for_each(
                |(entity, task, engine, mut velocity)| match move_to_entity::move_to_position(
                    entity,
                    task.global_position,
                    0.0,
                    true,
                    &all_transforms,
                    engine,
                    &mut velocity,
                    delta_seconds,
                ) {
                    TaskResult::Ongoing => {}
                    TaskResult::Finished | TaskResult::Aborted => task_completions
                        .lock()
                        .unwrap()
                        .push(TaskCompletedEvent::<MoveToPosition>::new(entity.into())),
                },
            );

        send_completion_events(event_writer, task_completions);
    }

    pub(crate) fn cancel_task_inside_queue() {
        // Nothing needs to be done
    }

    pub(crate) fn abort_running_task() {
        // Nothing needs to be done
    }
}

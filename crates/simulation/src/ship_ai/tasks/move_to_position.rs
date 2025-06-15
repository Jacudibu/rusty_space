use crate::ship_ai::TaskComponent;
use crate::ship_ai::ship_task::ShipTask;
use crate::ship_ai::task_creation::{
    GeneralPathfindingArgs, TaskCreationEventHandler, create_preconditions_and_move_to_sector,
};
use crate::ship_ai::task_result::TaskResult;
use crate::ship_ai::tasks::{move_to_entity, send_completion_events};
use bevy::ecs::system::{StaticSystemParam, SystemParam};
use bevy::prelude::{BevyError, Entity, EventWriter, Query, Res, Time};
use common::components::Engine;
use common::components::ship_velocity::ShipVelocity;
use common::components::task_kind::TaskKind;
use common::components::task_queue::TaskQueue;
use common::events::task_events::{InsertTaskIntoQueueCommand, TaskCompletedEvent};
use common::simulation_transform::SimulationTransform;
use common::types::ship_tasks::MoveToPosition;
use std::collections::VecDeque;
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

#[derive(SystemParam)]
pub struct MoveToPositionArgs {}

impl TaskCreationEventHandler<MoveToPosition, MoveToPositionArgs> for MoveToPosition {
    fn create_tasks_for_command(
        event: &InsertTaskIntoQueueCommand<MoveToPosition>,
        task_queue: &TaskQueue,
        general_pathfinding_args: &GeneralPathfindingArgs,
        _args: &mut StaticSystemParam<MoveToPositionArgs>,
    ) -> Result<VecDeque<TaskKind>, BevyError> {
        let mut new_tasks = create_preconditions_and_move_to_sector(
            event.entity,
            task_queue,
            event.task_data.sector_position.sector,
            None, // TODO: Once the underlying logic uses local space
            general_pathfinding_args,
        )?;

        new_tasks.push_back(TaskKind::MoveToPosition {
            data: event.task_data.clone(),
        });

        Ok(new_tasks)
    }
}

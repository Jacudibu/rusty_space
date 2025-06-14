use crate::ship_ai::TaskComponent;
use crate::ship_ai::ship_task::ShipTask;
use crate::ship_ai::task_creation::{
    GeneralPathfindingArgs, TaskCreation, create_preconditions_and_move_to_sector,
};
use crate::ship_ai::task_result::TaskResult;
use crate::ship_ai::tasks::send_completion_events;
use bevy::ecs::system::{StaticSystemParam, SystemParam};
use bevy::prelude::{BevyError, Entity, EventWriter, Query};
use common::components::InSector;
use common::components::task_kind::TaskKind;
use common::components::task_queue::TaskQueue;
use common::events::task_events::{InsertTaskIntoQueueCommand, TaskCompletedEvent};
use common::types::entity_wrappers::SectorEntity;
use common::types::ship_tasks::MoveToSector;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

impl TaskComponent for ShipTask<MoveToSector> {
    fn can_be_cancelled_while_active() -> bool {
        true
    }
}

/// ...if we aren't, something went terribly wrong!
fn verify_that_we_are_in_target_sector(
    current_sector: SectorEntity,
    in_sector: SectorEntity,
) -> TaskResult {
    if current_sector != in_sector {
        todo!("Prerequisites not met, move back into queue!")
    }

    panic!("This task should never be run directly!");
    // TaskResult::Finished
}

impl ShipTask<MoveToSector> {
    pub fn run_tasks(
        event_writer: EventWriter<TaskCompletedEvent<MoveToSector>>,
        mut ships: Query<(Entity, &Self, &InSector)>,
    ) {
        let task_completions = Arc::new(Mutex::new(Vec::<TaskCompletedEvent<MoveToSector>>::new()));

        ships.par_iter_mut().for_each(|(entity, task, in_sector)| {
            match verify_that_we_are_in_target_sector(task.sector, in_sector.sector) {
                TaskResult::Ongoing => {}
                TaskResult::Finished | TaskResult::Aborted => task_completions
                    .lock()
                    .unwrap()
                    .push(TaskCompletedEvent::<MoveToSector>::new(entity.into())),
            }
        });

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
pub struct MoveToSectorArgs {}

impl TaskCreation<MoveToSector, MoveToSectorArgs> for MoveToSector {
    fn create_tasks_for_command(
        event: &InsertTaskIntoQueueCommand<MoveToSector>,
        task_queue: &TaskQueue,
        general_pathfinding_args: &GeneralPathfindingArgs,
        _args: &mut StaticSystemParam<MoveToSectorArgs>,
    ) -> Result<VecDeque<TaskKind>, BevyError> {
        let new_tasks = create_preconditions_and_move_to_sector(
            event.entity,
            task_queue,
            event.task_data.sector,
            None,
            general_pathfinding_args,
        )?;

        Ok(new_tasks)
    }
}

use crate::TaskComponent;
use crate::task_lifecycle_traits::task_cancellation_active::TaskCancellationForActiveTaskEventHandler;
use crate::task_lifecycle_traits::task_cancellation_in_queue::TaskCancellationForTaskInQueueEventHandler;
use crate::task_lifecycle_traits::task_completed::TaskCompletedEventHandler;
use crate::task_lifecycle_traits::task_creation::{
    GeneralPathfindingArgs, TaskCreationEventHandler,
};
use crate::task_lifecycle_traits::task_started::TaskStartedEventHandler;
use crate::task_lifecycle_traits::task_update_runner::TaskUpdateRunner;
use crate::utility::ship_task::ShipTask;
use crate::utility::task_preconditions::create_preconditions_and_move_to_sector;
use crate::utility::task_result::TaskResult;
use bevy::ecs::system::{StaticSystemParam, SystemParam};
use bevy::prelude::{BevyError, Entity, Query};
use common::components::InSector;
use common::components::task_kind::TaskKind;
use common::components::task_queue::TaskQueue;
use common::events::task_events::{InsertTaskIntoQueueCommand, TaskCompletedEvent};
use common::types::entity_wrappers::SectorEntity;
use common::types::ship_tasks::MoveToSector;
use std::collections::VecDeque;
use std::ops::DerefMut;
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

    // TODO: this may change with the new task queue system
    panic!(
        "This task should never be run directly - we don't add it in the InsertTaskToQueueCommand!"
    );
    // TaskResult::Finished
}

#[derive(SystemParam)]
pub struct TaskUpdateRunnerArgsMut<'w, 's> {
    ships: Query<'w, 's, (Entity, &'static ShipTask<MoveToSector>, &'static InSector)>,
}

impl<'w, 's> TaskUpdateRunner<'w, 's, Self> for MoveToSector {
    type Args = ();
    type ArgsMut = TaskUpdateRunnerArgsMut<'w, 's>;

    fn run_all_tasks(
        _args: StaticSystemParam<Self::Args>,
        mut args_mut: StaticSystemParam<Self::ArgsMut>,
    ) -> Result<Arc<Mutex<Vec<TaskCompletedEvent<MoveToSector>>>>, BevyError> {
        let args_mut = args_mut.deref_mut();

        let task_completions = Arc::new(Mutex::new(Vec::<TaskCompletedEvent<MoveToSector>>::new()));

        args_mut
            .ships
            .par_iter_mut()
            .for_each(|(entity, task, in_sector)| {
                match verify_that_we_are_in_target_sector(task.sector, in_sector.sector) {
                    TaskResult::Ongoing => {}
                    TaskResult::Finished | TaskResult::Aborted => task_completions
                        .lock()
                        .unwrap()
                        .push(TaskCompletedEvent::<MoveToSector>::new(entity.into())),
                }
            });

        Ok(task_completions)
    }
}

impl<'w, 's> TaskCreationEventHandler<'w, 's, Self> for MoveToSector {
    type Args = ();
    type ArgsMut = ();

    fn create_tasks_for_command(
        event: &InsertTaskIntoQueueCommand<MoveToSector>,
        task_queue: &TaskQueue,
        general_pathfinding_args: &GeneralPathfindingArgs,
        _args: &StaticSystemParam<Self::Args>,
        _args_mut: &mut StaticSystemParam<Self::ArgsMut>,
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

impl<'w, 's> TaskStartedEventHandler<'w, 's, Self> for MoveToSector {
    type Args = ();
    type ArgsMut = ();

    fn skip_started() -> bool {
        true
    }
}

impl<'w, 's> TaskCancellationForTaskInQueueEventHandler<'w, 's, Self> for MoveToSector {
    type Args = ();
    type ArgsMut = ();

    fn can_task_be_cancelled_while_in_queue() -> bool {
        true
    }

    fn skip_cancelled_in_queue() -> bool {
        true
    }
}

impl<'w, 's> TaskCancellationForActiveTaskEventHandler<'w, 's, Self> for MoveToSector {
    type Args = ();
    type ArgsMut = ();

    fn can_task_be_cancelled_while_active() -> bool {
        true
    }

    fn skip_cancelled_while_active() -> bool {
        true
    }
}

impl<'w, 's> TaskCompletedEventHandler<'w, 's, Self> for MoveToSector {
    type Args = ();
    type ArgsMut = ();

    fn skip_completed() -> bool {
        true
    }
}

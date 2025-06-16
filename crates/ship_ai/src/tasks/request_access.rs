use crate::TaskComponent;
use crate::task_lifecycle_traits::task_cancellation_active::TaskCancellationForActiveTaskEventHandler;
use crate::task_lifecycle_traits::task_cancellation_in_queue::TaskCancellationForTaskInQueueEventHandler;
use crate::task_lifecycle_traits::task_completed::TaskCompletedEventHandler;
use crate::task_lifecycle_traits::task_creation::{
    GeneralPathfindingArgs, TaskCreationError, TaskCreationErrorReason, TaskCreationEventHandler,
};
use crate::task_lifecycle_traits::task_started::TaskStartedEventHandler;
use crate::task_lifecycle_traits::task_update_runner::TaskUpdateRunner;
use crate::utility::ship_task::ShipTask;
use bevy::ecs::system::{StaticSystemParam, SystemParam};
use bevy::prelude::{BevyError, Entity, Query};
use common::components::DockingBay;
use common::components::interaction_queue::{InteractionQueue, InteractionQueueResult};
use common::components::task_kind::TaskKind;
use common::components::task_queue::TaskQueue;
use common::events::task_events::{InsertTaskIntoQueueCommand, TaskCompletedEvent};
use common::types::ship_tasks::{AwaitingSignal, RequestAccess, RequestAccessGoal};
use std::collections::VecDeque;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};

impl TaskComponent for ShipTask<RequestAccess> {
    fn can_be_cancelled_while_active() -> bool {
        false
    }
}

fn access_dock(
    entity: Entity,
    task: &ShipTask<RequestAccess>,
    all_docking_bays: &mut Query<&mut DockingBay>,
) -> Result<InteractionQueueResult, BevyError> {
    let Ok(mut docking_bay) = all_docking_bays.get_mut(task.target.into()) else {
        todo!("In case no entity to dock at was found, cancel task");
    };

    Ok(docking_bay.try_dock(entity.into()))
}

fn access_undock(
    entity: Entity,
    task: &ShipTask<RequestAccess>,
    all_docking_bays: &mut Query<&mut DockingBay>,
) -> Result<InteractionQueueResult, BevyError> {
    let Ok(mut docking_bay) = all_docking_bays.get_mut(task.target.into()) else {
        todo!("In case no entity to dock at was found, cancel task");
    };

    Ok(docking_bay.try_undock(entity.into()))
}

fn access_planet_orbit(
    entity: Entity,
    task: &ShipTask<RequestAccess>,
    all_interaction_queues: &mut Query<&mut InteractionQueue>,
) -> Result<InteractionQueueResult, BevyError> {
    let Ok(mut interaction_queue) = all_interaction_queues.get_mut(task.target.into()) else {
        panic!("Planets cannot be destroyed, so this should never happen!")
    };

    Ok(interaction_queue.try_start_interaction(entity.into()))
}

#[derive(SystemParam)]
pub struct TaskUpdateRunnerArgsMut<'w, 's> {
    all_ships_with_task: Query<
        'w,
        's,
        (
            Entity,
            &'static ShipTask<RequestAccess>,
            &'static mut TaskQueue,
        ),
    >,
    all_interaction_queues: Query<'w, 's, &'static mut InteractionQueue>,
    all_docking_bays: Query<'w, 's, &'static mut DockingBay>,
}

impl<'w, 's> TaskUpdateRunner<'w, 's, Self> for RequestAccess {
    type Args = ();
    type ArgsMut = TaskUpdateRunnerArgsMut<'w, 's>;

    fn run_all_tasks(
        _args: StaticSystemParam<Self::Args>,
        mut args_mut: StaticSystemParam<Self::ArgsMut>,
    ) -> Result<Arc<Mutex<Vec<TaskCompletedEvent<RequestAccess>>>>, BevyError> {
        let args_mut = args_mut.deref_mut();
        let mut task_completions = Vec::default();

        for (entity, task, mut task_queue) in args_mut.all_ships_with_task.iter_mut() {
            let result = match task.goal {
                RequestAccessGoal::Docking => {
                    access_dock(entity, task, &mut args_mut.all_docking_bays)
                }
                RequestAccessGoal::Undocking => {
                    access_undock(entity, task, &mut args_mut.all_docking_bays)
                }
                RequestAccessGoal::PlanetOrbit => {
                    access_planet_orbit(entity, task, &mut args_mut.all_interaction_queues)
                }
            }?;

            match result {
                InteractionQueueResult::ProceedImmediately => {}
                InteractionQueueResult::EnteredQueuePleaseAddAwaitingSignalToQueue => {
                    task_queue.push_front(TaskKind::AwaitingSignal {
                        data: AwaitingSignal { from: task.target },
                    });
                }
            }

            task_completions.push(TaskCompletedEvent::<RequestAccess>::new(entity.into()));
        }

        // This is the only task where this return type *is* awkward.
        Ok(Arc::new(Mutex::new(task_completions)))
    }
}

impl<'w, 's> TaskCreationEventHandler<'w, 's, Self> for RequestAccess {
    type Args = ();
    type ArgsMut = ();

    fn create_tasks_for_command(
        event: &InsertTaskIntoQueueCommand<RequestAccess>,
        _task_queue: &TaskQueue,
        _general_pathfinding_args: &GeneralPathfindingArgs,
        _args: &StaticSystemParam<Self::Args>,
        _args_mut: &mut StaticSystemParam<Self::ArgsMut>,
    ) -> Result<VecDeque<TaskKind>, BevyError> {
        // We really don't want to create this task manually, as it is just a precondition for tasks involving queues.
        Err(BevyError::from(TaskCreationError {
            entity: event.entity,
            reason: TaskCreationErrorReason::CreationOfThisTaskIsNotSupported,
        }))
    }
}

impl<'w, 's> TaskStartedEventHandler<'w, 's, Self> for RequestAccess {
    type Args = ();
    type ArgsMut = ();

    fn skip_started() -> bool {
        true
    }
}

impl<'w, 's> TaskCancellationForTaskInQueueEventHandler<'w, 's, Self> for RequestAccess {
    type Args = ();
    type ArgsMut = ();

    fn can_task_be_cancelled_while_in_queue() -> bool {
        true
    }

    fn skip_cancelled_in_queue() -> bool {
        true
    }
}

impl<'w, 's> TaskCancellationForActiveTaskEventHandler<'w, 's, Self> for RequestAccess {
    type Args = ();
    type ArgsMut = ();

    fn can_task_be_cancelled_while_active() -> bool {
        // This task lives only one tick, so it's not worth the headache to support this...
        false
    }
}

impl<'w, 's> TaskCompletedEventHandler<'w, 's, Self> for RequestAccess {
    type Args = ();
    type ArgsMut = ();

    fn skip_completed() -> bool {
        true
    }
}

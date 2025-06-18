use crate::TaskComponent;
use crate::task_lifecycle_traits::task_cancellation_active::TaskCancellationForActiveTaskEventHandler;
use crate::task_lifecycle_traits::task_cancellation_in_queue::TaskCancellationForTaskInQueueEventHandler;
use crate::task_lifecycle_traits::task_completed::TaskCompletedEventHandler;
use crate::task_lifecycle_traits::task_creation::{
    GeneralPathfindingArgs, TaskCreationEventHandler,
};
use crate::task_lifecycle_traits::task_started::TaskStartedEventHandler;
use crate::task_lifecycle_traits::task_update_runner::TaskUpdateRunner;
use crate::tasks::send_completion_events;
use crate::utility::ship_task::ShipTask;
use bevy::ecs::system::{StaticSystemParam, SystemParam};
use bevy::prelude::{BevyError, EventReader, EventWriter, Query};
use common::components::interaction_queue::InteractionQueue;
use common::components::task_kind::TaskKind;
use common::components::task_queue::TaskQueue;
use common::constants::BevyResult;
use common::events::send_signal_event::SendSignalEvent;
use common::events::task_events::{
    InsertTaskIntoQueueCommand, TaskCanceledWhileActiveEvent, TaskCompletedEvent,
};
use common::types::ship_tasks::AwaitingSignal;
use std::collections::VecDeque;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};

impl TaskComponent for ShipTask<AwaitingSignal> {
    fn can_be_cancelled_while_active() -> bool {
        true
    }
}

#[derive(SystemParam)]
pub struct TaskUpdateRunnerArgsMut<'w, 's> {
    signal_reader: EventReader<'w, 's, SendSignalEvent>,
}

impl<'w, 's> TaskUpdateRunner<'w, 's, Self> for AwaitingSignal {
    type Args = ();
    type ArgsMut = TaskUpdateRunnerArgsMut<'w, 's>;

    fn run_all_tasks(
        _args: StaticSystemParam<Self::Args>,
        _args_mut: StaticSystemParam<Self::ArgsMut>,
    ) -> Result<Arc<Mutex<Vec<TaskCompletedEvent<Self>>>>, BevyError> {
        panic!("This should never be called!")
    }

    fn update(
        event_writer: EventWriter<TaskCompletedEvent<Self>>,
        _args: StaticSystemParam<Self::Args>,
        mut args_mut: StaticSystemParam<Self::ArgsMut>,
    ) -> BevyResult {
        let args_mut = args_mut.deref_mut();

        let completions = args_mut
            .signal_reader
            .read()
            .map(|event| TaskCompletedEvent::<AwaitingSignal>::new(event.entity))
            .collect();

        send_completion_events(event_writer, Arc::new(Mutex::new(completions)));
        Ok(())
    }
}

impl<'w, 's> TaskCreationEventHandler<'w, 's, Self> for AwaitingSignal {
    type Args = ();
    type ArgsMut = ();

    fn create_tasks_for_command(
        _event: &InsertTaskIntoQueueCommand<Self>,
        _task_queue: &TaskQueue,
        _general_pathfinding_args: &GeneralPathfindingArgs,
        _args: &StaticSystemParam<Self::Args>,
        _args_mut: &mut StaticSystemParam<Self::ArgsMut>,
    ) -> Result<VecDeque<TaskKind>, BevyError> {
        panic!("Awaiting Signal cannot be created manually (yet!)")
    }
}

impl<'w, 's> TaskStartedEventHandler<'w, 's, Self> for AwaitingSignal {
    type Args = ();
    type ArgsMut = ();

    fn skip_started() -> bool {
        true
    }
}

impl<'w, 's> TaskCancellationForTaskInQueueEventHandler<'w, 's, Self> for AwaitingSignal {
    type Args = ();
    type ArgsMut = ();

    fn can_task_be_cancelled_while_in_queue() -> bool {
        true
    }

    fn skip_cancelled_in_queue() -> bool {
        true
    }
}

#[derive(SystemParam)]
pub struct TaskCancellationForActiveTaskArgsMut<'w, 's> {
    interaction_queues: Query<'w, 's, &'static mut InteractionQueue>,
}

impl<'w, 's> TaskCancellationForActiveTaskEventHandler<'w, 's, Self> for AwaitingSignal {
    type Args = ();
    type ArgsMut = TaskCancellationForActiveTaskArgsMut<'w, 's>;

    fn on_task_cancellation_while_in_active(
        event: &TaskCanceledWhileActiveEvent<Self>,
        _args: &StaticSystemParam<Self::Args>,
        args_mut: &mut StaticSystemParam<Self::ArgsMut>,
    ) -> Result<(), BevyError> {
        let args_mut = args_mut.deref_mut();

        let mut interaction_queue = args_mut
            .interaction_queues
            .get_mut(event.task_data.from.into())?;

        interaction_queue.remove_from_queue(event.entity);

        Ok(())
    }
}

impl<'w, 's> TaskCompletedEventHandler<'w, 's, Self> for AwaitingSignal {
    type Args = ();
    type ArgsMut = ();

    fn skip_completed() -> bool {
        true
    }
}

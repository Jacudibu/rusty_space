use crate::TaskComponent;
use crate::task_lifecycle_traits::task_cancellation_in_queue;
use crate::utility::ship_task::ShipTask;
use bevy::ecs::system::{StaticSystemParam, SystemParam};
use bevy::prelude::{BevyError, Commands, Event, EventReader, EventWriter, Query, info, warn};
use common::components::task_kind::TaskKind;
use common::components::task_queue::TaskQueue;
use common::constants::BevyResult;
use common::events::task_events::{
    AllTaskAbortedEventWriters, AllTaskCancelledEventWriters, TaskCanceledWhileActiveEvent,
};
use common::types::entity_wrappers::ShipEntity;
use common::types::ship_tasks::{
    AwaitingSignal, Construct, DockAtEntity, ExchangeWares, HarvestGas, MineAsteroid, MoveToEntity,
    MoveToPosition, MoveToSector, RequestAccess, ShipTaskData, Undock, UseGate,
};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

/// Send this event in order to request a ship to stop doing whatever it is doing right now, and also clear its entire task queue.
/// Tasks which are aborted are also getting cancelled, so there's no reason to implement cancellation logic within the abortion handler.
#[derive(Event)]
pub struct TaskCancellationWhileActiveRequest {
    /// The affected entity.
    pub entity: ShipEntity,
}

/// Default error used during [TaskCancellationForActiveTaskEventHandler].
#[derive(Debug)]
struct TaskCancellationForActiveTaskNotImplementedError<TaskData: ShipTaskData> {
    pub entity: ShipEntity,
    pub task_data: TaskData,
}

impl<TaskData: ShipTaskData> Display
    for TaskCancellationForActiveTaskNotImplementedError<TaskData>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl<TaskData: ShipTaskData> Error for TaskCancellationForActiveTaskNotImplementedError<TaskData> {}

/// This trait needs to be implemented for all tasks.
///
/// Provides a blanket implementation which prints errors in case we attempt to cancel a task which
/// is not supposed to be cancelled
pub(crate) trait TaskCancellationForActiveTaskEventHandler<'w, 's, TaskData: ShipTaskData> {
    /// The immutable arguments used when calling the functions of this trait.
    type Args: SystemParam;
    /// The mutable arguments used when calling the functions of this trait.
    type ArgsMut: SystemParam;

    /// Whether this task may be cancelled while it is actively being executed.
    fn can_task_be_cancelled_while_active() -> bool {
        false
    }

    /// If set to true, the event listener system won't be registered at all. Only do this if there's no custom logic necessary.
    fn skip_cancelled_while_active() -> bool {
        true
    }

    /// You need to either override this or set [Self::skip_cancelled_while_active] to true so the event listener won't be registered.
    fn on_task_cancellation_while_in_active(
        event: &TaskCanceledWhileActiveEvent<TaskData>,
        _args: &StaticSystemParam<Self::Args>,
        _args_mut: &mut StaticSystemParam<Self::ArgsMut>,
    ) -> Result<(), BevyError> {
        Err(BevyError::from(
            TaskCancellationForActiveTaskNotImplementedError {
                entity: event.entity,
                task_data: event.task_data.clone(),
            },
        ))
    }

    /// Listens to [TaskCanceledWhileActiveEvent]s and runs [Self::on_task_cancellation_while_in_active] for each.
    /// Usually you don't need to reimplement this.
    fn cancellation_while_active_event_listener(
        mut commands: Commands,
        mut events: EventReader<TaskCanceledWhileActiveEvent<TaskData>>,
        args: StaticSystemParam<Self::Args>,
        mut args_mut: StaticSystemParam<Self::ArgsMut>,
    ) -> BevyResult {
        for event in events.read() {
            Self::on_task_cancellation_while_in_active(event, &args, &mut args_mut)?;
            commands
                .entity(event.entity.into())
                .remove::<ShipTask<TaskData>>();
        }

        Ok(())
    }
}

pub fn can_task_be_cancelled_while_active(task: &TaskKind) -> bool {
    match task {
        TaskKind::AwaitingSignal { .. } => {
            ShipTask::<AwaitingSignal>::can_be_cancelled_while_active()
        }
        TaskKind::Construct { .. } => ShipTask::<Construct>::can_be_cancelled_while_active(),
        TaskKind::RequestAccess { .. } => {
            ShipTask::<RequestAccess>::can_be_cancelled_while_active()
        }
        TaskKind::DockAtEntity { .. } => ShipTask::<DockAtEntity>::can_be_cancelled_while_active(),
        TaskKind::Undock { .. } => ShipTask::<Undock>::can_be_cancelled_while_active(),
        TaskKind::ExchangeWares { .. } => {
            ShipTask::<ExchangeWares>::can_be_cancelled_while_active()
        }
        TaskKind::MoveToEntity { .. } => ShipTask::<MoveToEntity>::can_be_cancelled_while_active(),
        TaskKind::MoveToPosition { .. } => {
            ShipTask::<MoveToPosition>::can_be_cancelled_while_active()
        }
        TaskKind::MoveToSector { .. } => ShipTask::<MoveToSector>::can_be_cancelled_while_active(),
        TaskKind::UseGate { .. } => ShipTask::<UseGate>::can_be_cancelled_while_active(),
        TaskKind::MineAsteroid { .. } => ShipTask::<MineAsteroid>::can_be_cancelled_while_active(),
        TaskKind::HarvestGas { .. } => ShipTask::<HarvestGas>::can_be_cancelled_while_active(),
    }
}

/// Completely clears task queues.
pub(crate) fn handle_task_cancellation_while_active_requests(
    mut events: EventReader<TaskCancellationWhileActiveRequest>,
    mut all_task_queues: Query<&mut TaskQueue>,
    mut event_writers: AllTaskAbortedEventWriters,
    mut cancellation_event_writers: AllTaskCancelledEventWriters,
) -> BevyResult {
    for event in events.read() {
        let mut queue = all_task_queues.get_mut(event.entity.into())?;

        let Some(active_task) = queue.active_task.clone() else {
            info!(
                "Abort task was called on an entity without active task: {}",
                event.entity
            );
            continue;
        };

        if !can_task_be_cancelled_while_active(&active_task) {
            warn!(
                "Abort task was called on an entity with a task which cannot be aborted: {}",
                event.entity
            );
            continue;
        }

        match active_task {
            TaskKind::AwaitingSignal { data } => {
                write_event(&mut event_writers.awaiting_signal, event.entity, data)
            }
            TaskKind::Construct { data } => {
                write_event(&mut event_writers.construct, event.entity, data);
            }
            TaskKind::RequestAccess { data } => {
                write_event(&mut event_writers.request_access, event.entity, data)
            }
            TaskKind::DockAtEntity { data } => {
                write_event(&mut event_writers.dock_at_entity, event.entity, data)
            }
            TaskKind::Undock { data } => write_event(&mut event_writers.undock, event.entity, data),
            TaskKind::ExchangeWares { data } => {
                write_event(&mut event_writers.exchange_wares, event.entity, data)
            }
            TaskKind::MoveToEntity { data } => {
                write_event(&mut event_writers.move_to_entity, event.entity, data)
            }
            TaskKind::MoveToPosition { data } => {
                write_event(&mut event_writers.move_to_position, event.entity, data)
            }
            TaskKind::MoveToSector { data } => {
                write_event(&mut event_writers.move_to_sector, event.entity, data)
            }
            TaskKind::UseGate { data } => {
                write_event(&mut event_writers.use_gate, event.entity, data)
            }
            TaskKind::MineAsteroid { data } => {
                write_event(&mut event_writers.mine_asteroid, event.entity, data)
            }
            TaskKind::HarvestGas { data } => {
                write_event(&mut event_writers.harvest_gas, event.entity, data)
            }
        }

        task_cancellation_in_queue::send_cancellation_event(
            &mut cancellation_event_writers,
            event.entity,
            queue.active_task.clone().unwrap(),
        );
        for task in queue.queue.split_off(0) {
            task_cancellation_in_queue::send_cancellation_event(
                &mut cancellation_event_writers,
                event.entity,
                task,
            );
        }

        queue.active_task = None;
    }

    Ok(())
}

#[inline]
fn write_event<T: ShipTaskData>(
    event_writer: &mut EventWriter<TaskCanceledWhileActiveEvent<T>>,
    entity: ShipEntity,
    data: T,
) {
    event_writer.write(TaskCanceledWhileActiveEvent::new(entity, data));
}

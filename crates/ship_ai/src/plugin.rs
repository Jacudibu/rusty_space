use crate::behaviors;
use crate::task_lifecycle_traits::task_cancellation_active::{
    TaskCancellationForActiveTaskEventHandler, TaskCancellationWhileActiveRequest,
};
use crate::task_lifecycle_traits::task_cancellation_in_queue::{
    TaskCancellationForTaskInQueueEventHandler, TaskCancellationWhileInQueueRequest,
};
use crate::task_lifecycle_traits::task_completed::TaskCompletedEventHandler;
use crate::task_lifecycle_traits::task_creation::TaskCreationEventHandler;
use crate::task_lifecycle_traits::task_started::TaskStartedEventHandler;
use crate::task_lifecycle_traits::task_update_runner::TaskUpdateRunner;
use crate::task_lifecycle_traits::{task_cancellation_active, task_cancellation_in_queue};
use crate::utility::stop_idle_ships;
use bevy::app::App;
use bevy::prelude::{
    FixedPostUpdate, FixedUpdate, IntoScheduleConfigs, Plugin, PreUpdate, Update, in_state,
    on_event,
};
use common::events::send_signal_event::SendSignalEvent;
use common::events::task_events::{
    InsertTaskIntoQueueCommand, TaskCanceledWhileActiveEvent, TaskCanceledWhileInQueueEvent,
    TaskCompletedEvent, TaskStartedEvent,
};
use common::states::SimulationState;
use common::system_sets::CustomSystemSets;
use common::types::ship_tasks::{
    AwaitingSignal, Construct, DockAtEntity, ExchangeWares, HarvestGas, MineAsteroid, MoveToEntity,
    MoveToPosition, MoveToSector, RequestAccess, ShipTaskData, Undock, UseGate,
};

pub struct ShipAiPlugin;
impl Plugin for ShipAiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SendSignalEvent>();

        register_task_lifecycle::<AwaitingSignal>(app);
        register_task_lifecycle::<Construct>(app);
        register_task_lifecycle::<DockAtEntity>(app);
        register_task_lifecycle::<ExchangeWares>(app);
        register_task_lifecycle::<HarvestGas>(app);
        register_task_lifecycle::<MineAsteroid>(app);
        register_task_lifecycle::<MoveToEntity>(app);
        register_task_lifecycle::<MoveToPosition>(app);
        register_task_lifecycle::<MoveToSector>(app);
        register_task_lifecycle::<RequestAccess>(app);
        register_task_lifecycle::<Undock>(app);
        register_task_lifecycle::<UseGate>(app);

        app.add_systems(
            FixedUpdate,
            (
                behaviors::auto_construct::handle_idle_ships,
                behaviors::auto_trade::handle_idle_ships,
                behaviors::auto_harvest::handle_idle_ships,
                behaviors::auto_mine::handle_idle_ships.before(CustomSystemSets::RespawnAsteroids),
            )
                .run_if(in_state(SimulationState::Running)),
        );

        app.add_systems(
            FixedUpdate,
            (stop_idle_ships::stop_idle_ships).run_if(in_state(SimulationState::Running)),
        );

        enable_cancelling_active_tasks(app);
        enable_cancelling_tasks_in_queue(app);
    }
}

fn enable_cancelling_active_tasks(app: &mut App) {
    app.add_systems(
        Update,
        task_cancellation_active::handle_task_cancellation_while_active_requests,
    );

    app.add_event::<TaskCancellationWhileActiveRequest>();
}

fn enable_cancelling_tasks_in_queue(app: &mut App) {
    app.add_systems(
        Update,
        task_cancellation_in_queue::handle_task_cancellation_while_in_queue_requests
            .before(task_cancellation_active::handle_task_cancellation_while_active_requests),
    );

    app.add_event::<TaskCancellationWhileInQueueRequest>();
}

fn register_task_lifecycle<Task>(app: &mut App)
where
    Task: ShipTaskData
        + TaskCreationEventHandler<'static, 'static, Task>
        + TaskStartedEventHandler<'static, 'static, Task>
        + TaskCancellationForTaskInQueueEventHandler<'static, 'static, Task>
        + TaskUpdateRunner<'static, 'static, Task>
        + TaskCancellationForActiveTaskEventHandler<'static, 'static, Task>
        + TaskCompletedEventHandler<'static, 'static, Task>,
{
    app.add_event::<InsertTaskIntoQueueCommand<Task>>();
    app.add_event::<TaskCanceledWhileInQueueEvent<Task>>();
    app.add_event::<TaskStartedEvent<Task>>();
    app.add_event::<TaskCanceledWhileActiveEvent<Task>>();
    app.add_event::<TaskCompletedEvent<Task>>();

    if !Task::skip_cancelled_in_queue() {
        app.add_systems(
            PreUpdate,
            Task::cancellation_while_in_queue_event_listener
                .run_if(on_event::<TaskCanceledWhileInQueueEvent<Task>>),
        );
    }

    if !Task::skip_cancelled_while_active() {
        app.add_systems(
            PreUpdate,
            Task::cancellation_while_active_event_listener
                .run_if(on_event::<TaskCanceledWhileActiveEvent<Task>>),
        );
    }

    app.add_systems(Update, Task::task_creation_event_listener);

    // TODO: There must be *some* cleaner way to do this?
    if Task::skip_completed() {
        app.add_systems(
            FixedUpdate,
            (
                Task::update,
                Task::remove_completed_task_and_start_next_one
                    .run_if(on_event::<TaskCompletedEvent<Task>>),
            )
                .chain()
                .run_if(in_state(SimulationState::Running)),
        );
    } else {
        app.add_systems(
            FixedUpdate,
            (
                Task::update,
                (
                    Task::task_completed_event_listener,
                    Task::remove_completed_task_and_start_next_one,
                )
                    .chain()
                    .run_if(on_event::<TaskCompletedEvent<Task>>),
            )
                .chain()
                .run_if(in_state(SimulationState::Running)),
        );
    };

    if !Task::skip_started() {
        app.add_systems(
            FixedPostUpdate,
            Task::task_started_event_listener.run_if(in_state(SimulationState::Running)),
        );
    }
}

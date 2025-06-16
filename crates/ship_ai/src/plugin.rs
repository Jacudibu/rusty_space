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
use crate::tasks::apply_next_task;
use crate::utility::ship_task::ShipTask;
use crate::utility::stop_idle_ships;
use bevy::app::App;
use bevy::log::error;
use bevy::prelude::{
    Commands, EventReader, FixedPostUpdate, FixedPreUpdate, FixedUpdate, IntoScheduleConfigs,
    IntoSystem, Plugin, PreUpdate, Query, SystemSet, Update, With, in_state, on_event,
};
use common::components::task_queue::TaskQueue;
use common::events::task_events::{
    AllTaskStartedEventWriters, InsertTaskIntoQueueCommand, TaskCanceledWhileActiveEvent,
    TaskCanceledWhileInQueueEvent, TaskCompletedEvent, TaskStartedEvent,
};
use common::states::SimulationState;
use common::system_sets::CustomSystemSets;
use common::types::ship_tasks::{
    AwaitingSignal, Construct, DockAtEntity, ExchangeWares, HarvestGas, MineAsteroid, MoveToEntity,
    MoveToPosition, MoveToSector, RequestAccess, ShipTaskData, Undock, UseGate,
};

fn enable_insert_task_into_queue_commands(app: &mut App) {
    app.add_event::<InsertTaskIntoQueueCommand<Construct>>();
    app.add_event::<InsertTaskIntoQueueCommand<HarvestGas>>();
    app.add_event::<InsertTaskIntoQueueCommand<MineAsteroid>>();
    app.add_event::<InsertTaskIntoQueueCommand<MoveToSector>>();
}

// TODO: clean up once we reunify task registration
fn enable_abortion(app: &mut App) {
    app.add_systems(
        Update,
        task_cancellation_active::handle_task_cancellation_while_active_requests,
    );

    app.add_event::<TaskCancellationWhileActiveRequest>();
    app.add_event::<TaskCanceledWhileActiveEvent<AwaitingSignal>>();
    app.add_event::<TaskCanceledWhileActiveEvent<Construct>>();
    app.add_event::<TaskCanceledWhileActiveEvent<HarvestGas>>();
    app.add_event::<TaskCanceledWhileActiveEvent<MineAsteroid>>();
    app.add_event::<TaskCanceledWhileActiveEvent<MoveToEntity>>();
    app.add_event::<TaskCanceledWhileActiveEvent<MoveToSector>>();

    app.add_systems(
        FixedPreUpdate,
        (
            ShipTask::<Construct>::abort_running_task
                .run_if(on_event::<TaskCanceledWhileActiveEvent<Construct>>),
            ShipTask::<HarvestGas>::abort_running_task
                .run_if(on_event::<TaskCanceledWhileActiveEvent<HarvestGas>>),
            abort_tasks::<AwaitingSignal>
                .run_if(on_event::<TaskCanceledWhileActiveEvent<AwaitingSignal>>),
            abort_tasks::<Construct>.run_if(on_event::<TaskCanceledWhileActiveEvent<Construct>>),
            abort_tasks::<HarvestGas>.run_if(on_event::<TaskCanceledWhileActiveEvent<HarvestGas>>),
            abort_tasks::<MineAsteroid>
                .run_if(on_event::<TaskCanceledWhileActiveEvent<MineAsteroid>>),
            abort_tasks::<MoveToEntity>
                .run_if(on_event::<TaskCanceledWhileActiveEvent<MoveToEntity>>),
            abort_tasks::<MoveToSector>
                .run_if(on_event::<TaskCanceledWhileActiveEvent<MoveToSector>>),
        ),
    );
}

// TODO: clean up once we reunify task registration
fn enable_cancellation(app: &mut App) {
    app.add_systems(
        Update,
        task_cancellation_in_queue::handle_task_cancellation_while_in_queue_requests
            .before(task_cancellation_active::handle_task_cancellation_while_active_requests),
    );

    app.add_event::<TaskCancellationWhileInQueueRequest>();
    app.add_event::<TaskCanceledWhileInQueueEvent<AwaitingSignal>>();
    app.add_event::<TaskCanceledWhileInQueueEvent<Construct>>();
    app.add_event::<TaskCanceledWhileInQueueEvent<HarvestGas>>();
    app.add_event::<TaskCanceledWhileInQueueEvent<MineAsteroid>>();
    app.add_event::<TaskCanceledWhileInQueueEvent<MoveToEntity>>();
    app.add_event::<TaskCanceledWhileInQueueEvent<MoveToSector>>();

    app.add_systems(
        FixedPreUpdate,
        (ShipTask::<MineAsteroid>::cancel_task_inside_queue
            .run_if(on_event::<TaskCanceledWhileInQueueEvent<MineAsteroid>>),),
    );

    register_task_lifecycle::<DockAtEntity>(app);
    register_task_lifecycle::<ExchangeWares>(app);
    register_task_lifecycle::<MoveToPosition>(app);
    register_task_lifecycle::<RequestAccess>(app);
    register_task_lifecycle::<Undock>(app);
    register_task_lifecycle::<UseGate>(app);
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
                complete_tasks::<Task>.run_if(on_event::<TaskCompletedEvent<Task>>),
            )
                .chain()
                .run_if(in_state(SimulationState::Running)),
        );
    } else {
        app.add_systems(
            FixedUpdate,
            (
                Task::update,
                (Task::task_completed_event_listener, complete_tasks::<Task>)
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

pub struct ShipAiPlugin;
impl Plugin for ShipAiPlugin {
    fn build(&self, app: &mut App) {
        enable_insert_task_into_queue_commands(app);
        enable_abortion(app);
        enable_cancellation(app);

        app.add_systems(
            FixedUpdate,
            (
                behaviors::auto_construct::handle_idle_ships,
                behaviors::auto_trade::handle_idle_ships,
                behaviors::auto_harvest::handle_idle_ships,
                behaviors::auto_mine::handle_idle_ships
                    .before(CustomSystemSets::RespawnAsteroids)
                    .run_if(in_state(SimulationState::Running)),
            ),
        );

        app.add_event::<TaskCompletedEvent<Construct>>();
        app.add_event::<TaskStartedEvent<Construct>>();
        app.add_systems(Update, Construct::task_creation_event_listener);
        app.add_systems(
            FixedPostUpdate,
            (ShipTask::<Construct>::on_task_started,).run_if(in_state(SimulationState::Running)),
        );
        app.add_systems(
            FixedUpdate,
            (
                ShipTask::<Construct>::run_tasks,
                complete_tasks::<Construct>.run_if(on_event::<TaskCompletedEvent<Construct>>),
            )
                .chain()
                .run_if(in_state(SimulationState::Running)),
        );

        app.add_event::<TaskCompletedEvent<MoveToEntity>>();
        app.add_systems(
            FixedUpdate,
            (
                ShipTask::<MoveToEntity>::run_tasks,
                complete_tasks::<MoveToEntity>.run_if(on_event::<TaskCompletedEvent<MoveToEntity>>),
            )
                .chain()
                .run_if(in_state(SimulationState::Running)),
        );

        app.add_event::<TaskCompletedEvent<MoveToSector>>();
        app.add_systems(Update, MoveToSector::task_creation_event_listener);
        app.add_systems(
            FixedUpdate,
            (
                ShipTask::<MoveToSector>::run_tasks,
                complete_tasks::<MoveToSector>.run_if(on_event::<TaskCompletedEvent<MoveToSector>>),
            )
                .chain()
                .run_if(in_state(SimulationState::Running)),
        );

        app.add_event::<TaskCompletedEvent<MineAsteroid>>();
        app.add_event::<TaskStartedEvent<MineAsteroid>>();
        app.add_systems(Update, MineAsteroid::task_creation_event_listener);
        app.add_systems(
            FixedPostUpdate,
            ShipTask::<MineAsteroid>::on_task_started.run_if(in_state(SimulationState::Running)),
        );
        app.add_systems(
            FixedUpdate,
            (
                ShipTask::<MineAsteroid>::run_tasks,
                complete_tasks::<MineAsteroid>.run_if(on_event::<TaskCompletedEvent<MineAsteroid>>),
            )
                .chain()
                .run_if(in_state(SimulationState::Running)),
        );

        app.add_event::<TaskCompletedEvent<HarvestGas>>();
        app.add_event::<TaskStartedEvent<HarvestGas>>();
        app.add_systems(Update, HarvestGas::task_creation_event_listener);
        app.add_systems(
            FixedPostUpdate,
            ShipTask::<HarvestGas>::on_task_started.run_if(in_state(SimulationState::Running)),
        );
        app.add_systems(
            FixedUpdate,
            (
                ShipTask::<HarvestGas>::run_tasks,
                (
                    ShipTask::<HarvestGas>::complete_tasks,
                    complete_tasks::<HarvestGas>,
                )
                    .chain()
                    .run_if(on_event::<TaskCompletedEvent<HarvestGas>>),
            )
                .chain()
                .run_if(in_state(SimulationState::Running)),
        );

        app.add_systems(
            FixedUpdate,
            (stop_idle_ships::stop_idle_ships,).run_if(in_state(SimulationState::Running)),
        );

        app.add_event::<TaskCompletedEvent<AwaitingSignal>>();
        app.add_systems(
            FixedUpdate,
            (
                complete_tasks::<AwaitingSignal>
                    .run_if(on_event::<TaskCompletedEvent<AwaitingSignal>>)
                    .after(complete_tasks::<Undock>)
                    .after(complete_tasks::<HarvestGas>), // TODO: Could be replaced with a more general "disengage orbit" task or something alike
            )
                .run_if(in_state(SimulationState::Running)),
        );

        app.add_systems(FixedUpdate, stop_idle_ships::stop_idle_ships);
    }
}

fn complete_tasks<T: ShipTaskData>(
    mut commands: Commands,
    mut event_reader: EventReader<TaskCompletedEvent<T>>,
    mut all_ships_with_task: Query<&mut TaskQueue, With<ShipTask<T>>>,
    mut task_started_event_writers: AllTaskStartedEventWriters,
) {
    for event in event_reader.read() {
        if let Ok(mut queue) = all_ships_with_task.get_mut(event.entity.into()) {
            let entity = event.entity.into();
            let mut entity_commands = commands.entity(entity);
            entity_commands.remove::<ShipTask<T>>();
            apply_next_task(
                &mut queue,
                entity.into(),
                &mut entity_commands,
                &mut task_started_event_writers,
            );
        } else {
            error!(
                "Unable to find entity for generic task completion: {}",
                event.entity
            );
        }
    }
}

fn abort_tasks<T: ShipTaskData>(
    mut commands: Commands,
    mut event_reader: EventReader<TaskCanceledWhileActiveEvent<T>>,
) {
    for event in event_reader.read() {
        let entity = event.entity.into();
        let mut entity_commands = commands.entity(entity);
        entity_commands.remove::<ShipTask<T>>();
    }
}

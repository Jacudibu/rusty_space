use crate::ship_ai::ship_task::ShipTask;
use crate::ship_ai::task_abortion::TaskAbortionRequest;
use crate::ship_ai::task_cancellation::TaskCancellationRequest;
use crate::ship_ai::tasks::apply_next_task;
use crate::ship_ai::{behaviors, stop_idle_ships, task_abortion, task_cancellation};
use bevy::app::App;
use bevy::log::error;
use bevy::prelude::{
    Commands, EventReader, FixedPostUpdate, FixedPreUpdate, FixedUpdate, IntoScheduleConfigs,
    Plugin, Query, Res, Update, With, in_state, on_event,
};
use common::components::task_queue::TaskQueue;
use common::events::task_events::{
    AllTaskStartedEventWriters, TaskAbortedEvent, TaskCanceledEvent, TaskCompletedEvent,
    TaskStartedEvent,
};
use common::states::SimulationState;
use common::system_sets::CustomSystemSets;
use common::types::ship_tasks::{
    AwaitingSignal, Construct, DockAtEntity, ExchangeWares, HarvestGas, MineAsteroid, MoveToEntity,
    RequestAccess, ShipTaskData, Undock, UseGate,
};

// TODO: clean up once we reunify task registration
fn enable_abortion(app: &mut App) {
    app.add_systems(Update, task_abortion::handle_task_abortion_requests);

    app.add_event::<TaskAbortionRequest>();
    app.add_event::<TaskAbortedEvent<AwaitingSignal>>();
    app.add_event::<TaskAbortedEvent<Construct>>();
    app.add_event::<TaskAbortedEvent<ExchangeWares>>();
    app.add_event::<TaskAbortedEvent<DockAtEntity>>();
    app.add_event::<TaskAbortedEvent<HarvestGas>>();
    app.add_event::<TaskAbortedEvent<MineAsteroid>>();
    app.add_event::<TaskAbortedEvent<MoveToEntity>>();
    app.add_event::<TaskAbortedEvent<Undock>>();
    app.add_event::<TaskAbortedEvent<UseGate>>();
    app.add_event::<TaskAbortedEvent<RequestAccess>>();

    app.add_systems(
        FixedPreUpdate,
        (
            ShipTask::<Construct>::abort_running_task
                .run_if(on_event::<TaskAbortedEvent<Construct>>),
            ShipTask::<HarvestGas>::abort_running_task
                .run_if(on_event::<TaskAbortedEvent<HarvestGas>>),
            abort_tasks::<AwaitingSignal>.run_if(on_event::<TaskAbortedEvent<AwaitingSignal>>),
            abort_tasks::<Construct>.run_if(on_event::<TaskAbortedEvent<Construct>>),
            abort_tasks::<ExchangeWares>.run_if(on_event::<TaskAbortedEvent<ExchangeWares>>),
            abort_tasks::<DockAtEntity>.run_if(on_event::<TaskAbortedEvent<DockAtEntity>>),
            abort_tasks::<HarvestGas>.run_if(on_event::<TaskAbortedEvent<HarvestGas>>),
            abort_tasks::<MineAsteroid>.run_if(on_event::<TaskAbortedEvent<MineAsteroid>>),
            abort_tasks::<MoveToEntity>.run_if(on_event::<TaskAbortedEvent<MoveToEntity>>),
            abort_tasks::<Undock>.run_if(on_event::<TaskAbortedEvent<Undock>>),
            abort_tasks::<UseGate>.run_if(on_event::<TaskAbortedEvent<UseGate>>),
            abort_tasks::<RequestAccess>.run_if(on_event::<TaskAbortedEvent<RequestAccess>>),
        ),
    );
}

// TODO: clean up once we reunify task registration
fn enable_cancellation(app: &mut App) {
    app.add_systems(Update, task_cancellation::handle_task_cancellation_requests);

    app.add_event::<TaskCancellationRequest>();
    app.add_event::<TaskCanceledEvent<AwaitingSignal>>();
    app.add_event::<TaskCanceledEvent<Construct>>();
    app.add_event::<TaskCanceledEvent<ExchangeWares>>();
    app.add_event::<TaskCanceledEvent<DockAtEntity>>();
    app.add_event::<TaskCanceledEvent<HarvestGas>>();
    app.add_event::<TaskCanceledEvent<MineAsteroid>>();
    app.add_event::<TaskCanceledEvent<MoveToEntity>>();
    app.add_event::<TaskCanceledEvent<Undock>>();
    app.add_event::<TaskCanceledEvent<UseGate>>();
    app.add_event::<TaskCanceledEvent<RequestAccess>>();

    app.add_systems(
        FixedPreUpdate,
        (
            ShipTask::<ExchangeWares>::cancel_task_inside_queue
                .run_if(on_event::<TaskCanceledEvent<ExchangeWares>>),
            ShipTask::<MineAsteroid>::cancel_task_inside_queue
                .run_if(on_event::<TaskCanceledEvent<MineAsteroid>>),
        ),
    );
}

pub struct ShipAiPlugin;
impl Plugin for ShipAiPlugin {
    fn build(&self, app: &mut App) {
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

        app.add_event::<TaskCompletedEvent<Undock>>();
        app.add_event::<TaskStartedEvent<Undock>>();
        app.add_systems(
            FixedPostUpdate,
            ShipTask::<Undock>::on_task_started.run_if(in_state(SimulationState::Running)),
        );
        app.add_systems(
            FixedUpdate,
            (
                ShipTask::<Undock>::run_tasks,
                complete_tasks::<Undock>.run_if(on_event::<TaskCompletedEvent<Undock>>),
            )
                .chain()
                .run_if(in_state(SimulationState::Running)),
        );

        app.add_event::<TaskCompletedEvent<ExchangeWares>>();
        app.add_event::<TaskStartedEvent<ExchangeWares>>();
        app.add_systems(
            FixedPostUpdate,
            ShipTask::<ExchangeWares>::on_task_started.run_if(in_state(SimulationState::Running)),
        );
        app.add_systems(
            FixedUpdate,
            (
                ShipTask::<ExchangeWares>::run_tasks,
                (
                    ShipTask::<ExchangeWares>::complete_tasks,
                    complete_tasks::<ExchangeWares>,
                )
                    .chain()
                    .run_if(on_event::<TaskCompletedEvent<ExchangeWares>>),
            )
                .chain()
                .run_if(in_state(SimulationState::Running)),
        );

        app.add_event::<TaskCompletedEvent<UseGate>>();
        app.add_event::<TaskStartedEvent<UseGate>>();
        app.add_systems(
            FixedPostUpdate,
            ShipTask::<UseGate>::on_task_started.run_if(in_state(SimulationState::Running)),
        );
        app.add_systems(
            FixedUpdate,
            (
                ShipTask::<UseGate>::run_tasks,
                (
                    ShipTask::<UseGate>::complete_tasks,
                    complete_tasks::<UseGate>,
                )
                    .chain()
                    .run_if(on_event::<TaskCompletedEvent<UseGate>>),
            )
                .chain()
                .run_if(in_state(SimulationState::Running)),
        );

        app.add_event::<TaskCompletedEvent<Construct>>();
        app.add_event::<TaskStartedEvent<Construct>>();
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

        app.add_event::<TaskCompletedEvent<DockAtEntity>>();
        app.add_systems(
            FixedUpdate,
            (
                ShipTask::<DockAtEntity>::run_tasks,
                (
                    ShipTask::<DockAtEntity>::complete_tasks,
                    complete_tasks::<DockAtEntity>,
                )
                    .chain()
                    .run_if(on_event::<TaskCompletedEvent<DockAtEntity>>),
            )
                .chain()
                .run_if(in_state(SimulationState::Running)),
        );

        app.add_event::<TaskCompletedEvent<MineAsteroid>>();
        app.add_event::<TaskStartedEvent<MineAsteroid>>();
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

        app.add_event::<TaskCompletedEvent<RequestAccess>>();
        app.add_systems(
            FixedUpdate,
            (
                stop_idle_ships::stop_idle_ships,
                ShipTask::<RequestAccess>::run_tasks,
                complete_tasks::<RequestAccess>
                    .run_if(on_event::<TaskCompletedEvent<RequestAccess>>),
            )
                .run_if(in_state(SimulationState::Running)),
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

        app.add_systems(FixedUpdate, (stop_idle_ships::stop_idle_ships));
    }
}

fn complete_tasks<T: ShipTaskData + 'static>(
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

fn abort_tasks<T: ShipTaskData + 'static>(
    mut commands: Commands,
    mut event_reader: EventReader<TaskAbortedEvent<T>>,
) {
    for event in event_reader.read() {
        let entity = event.entity.into();
        let mut entity_commands = commands.entity(entity);
        entity_commands.remove::<ShipTask<T>>();
    }
}

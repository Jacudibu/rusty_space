use crate::simulation::asteroids;
use crate::simulation::prelude::{SimulationTime, TaskComponent, TaskQueue};
use crate::simulation::ship_ai::task_events::{
    AllTaskStartedEventWriters, TaskCompletedEvent, TaskStartedEvent,
};
use crate::simulation::ship_ai::tasks::{
    AwaitingSignal, ConstructTaskComponent, DockAtEntity, ExchangeWares, HarvestGas, MineAsteroid,
    MoveToEntity, RequestAccess, Undock, UseGate,
};
use crate::simulation::ship_ai::{behaviors, stop_idle_ships, tasks};
use crate::states::SimulationState;
use bevy::app::App;
use bevy::log::error;
use bevy::prelude::{
    Commands, EventReader, FixedPostUpdate, FixedUpdate, IntoScheduleConfigs, Plugin, Query, Res,
    With, in_state, on_event,
};

pub struct ShipAiPlugin;
impl Plugin for ShipAiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                behaviors::auto_construct::handle_idle_ships,
                behaviors::auto_trade::handle_idle_ships,
                behaviors::auto_harvest::handle_idle_ships,
                behaviors::auto_mine::handle_idle_ships
                    .before(asteroids::respawn_asteroids)
                    .run_if(in_state(SimulationState::Running)),
            ),
        );

        app.add_event::<TaskCompletedEvent<Undock>>();
        app.add_event::<TaskStartedEvent<Undock>>();
        app.add_systems(
            FixedPostUpdate,
            Undock::on_task_started.run_if(in_state(SimulationState::Running)),
        );
        app.add_systems(
            FixedUpdate,
            (
                Undock::run_tasks,
                complete_tasks::<Undock>.run_if(on_event::<TaskCompletedEvent<Undock>>),
            )
                .chain()
                .run_if(in_state(SimulationState::Running)),
        );

        app.add_event::<TaskCompletedEvent<ExchangeWares>>();
        app.add_event::<TaskStartedEvent<ExchangeWares>>();
        app.add_systems(
            FixedPostUpdate,
            ExchangeWares::on_task_started.run_if(in_state(SimulationState::Running)),
        );
        app.add_systems(
            FixedUpdate,
            (
                ExchangeWares::run_tasks,
                (
                    ExchangeWares::complete_tasks,
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
            UseGate::on_task_started.run_if(in_state(SimulationState::Running)),
        );
        app.add_systems(
            FixedUpdate,
            (
                UseGate::run_tasks,
                (UseGate::complete_tasks, complete_tasks::<UseGate>)
                    .chain()
                    .run_if(on_event::<TaskCompletedEvent<UseGate>>),
            )
                .chain()
                .run_if(in_state(SimulationState::Running)),
        );

        app.add_event::<TaskCompletedEvent<ConstructTaskComponent>>();
        app.add_event::<TaskStartedEvent<ConstructTaskComponent>>();
        app.add_systems(
            FixedPostUpdate,
            (ConstructTaskComponent::on_task_started,).run_if(in_state(SimulationState::Running)),
        );
        app.add_systems(
            FixedUpdate,
            (
                ConstructTaskComponent::run_tasks,
                complete_tasks::<ConstructTaskComponent>
                    .run_if(on_event::<TaskCompletedEvent<ConstructTaskComponent>>),
            )
                .chain()
                .run_if(in_state(SimulationState::Running)),
        );

        app.add_event::<TaskCompletedEvent<MoveToEntity>>();
        app.add_systems(
            FixedUpdate,
            (
                MoveToEntity::run_tasks,
                complete_tasks::<MoveToEntity>.run_if(on_event::<TaskCompletedEvent<MoveToEntity>>),
            )
                .chain()
                .run_if(in_state(SimulationState::Running)),
        );

        app.add_event::<TaskCompletedEvent<DockAtEntity>>();
        app.add_systems(
            FixedUpdate,
            (
                DockAtEntity::run_tasks,
                (DockAtEntity::complete_tasks, complete_tasks::<DockAtEntity>)
                    .chain()
                    .run_if(on_event::<TaskCompletedEvent<DockAtEntity>>),
            )
                .chain()
                .run_if(in_state(SimulationState::Running)),
        );

        app.add_event::<TaskCompletedEvent<MineAsteroid>>();
        app.add_systems(
            FixedUpdate,
            (
                MineAsteroid::run_tasks,
                complete_tasks::<MineAsteroid>.run_if(on_event::<TaskCompletedEvent<MineAsteroid>>),
            )
                .chain()
                .run_if(in_state(SimulationState::Running)),
        );

        app.add_event::<TaskCompletedEvent<HarvestGas>>();
        app.add_systems(
            FixedUpdate,
            (
                HarvestGas::run_tasks,
                (HarvestGas::complete_tasks, complete_tasks::<HarvestGas>)
                    .chain()
                    .run_if(on_event::<TaskCompletedEvent<HarvestGas>>),
            )
                .chain()
                .run_if(in_state(SimulationState::Running)),
        );

        app.add_event::<TaskCompletedEvent<AwaitingSignal>>();
        app.add_systems(
            FixedUpdate,
            (
                stop_idle_ships::stop_idle_ships,
                RequestAccess::run_tasks,
                complete_tasks::<AwaitingSignal>
                    .run_if(on_event::<TaskCompletedEvent<AwaitingSignal>>)
                    .after(complete_tasks::<Undock>)
                    .after(complete_tasks::<HarvestGas>), // TODO: Could be replaced with a more general "disengage orbit" task or something alike
            )
                .run_if(in_state(SimulationState::Running)),
        );
    }
}

fn complete_tasks<T: TaskComponent>(
    mut commands: Commands,
    mut event_reader: EventReader<TaskCompletedEvent<T>>,
    mut all_ships_with_task: Query<&mut TaskQueue, With<T>>,
    simulation_time: Res<SimulationTime>,
    mut task_started_event_writers: AllTaskStartedEventWriters,
) {
    let now = simulation_time.now();

    for event in event_reader.read() {
        if let Ok(mut queue) = all_ships_with_task.get_mut(event.entity.into()) {
            tasks::remove_task_and_add_next_in_queue::<T>(
                &mut commands,
                event.entity.into(),
                &mut queue,
                now,
                &mut task_started_event_writers,
            );
        } else {
            error!(
                "Unable to find entity for task completion: {}",
                event.entity
            );
        }
    }
}

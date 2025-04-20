use crate::simulation::asteroids;
use crate::simulation::prelude::{SimulationTime, TaskComponent, TaskQueue};
use crate::simulation::ship_ai::task_events::{AllTaskStartedEventWriters, TaskCompletedEvent, TaskStartedEvent};
use crate::simulation::ship_ai::tasks::{
    AwaitingSignal, ConstructTaskComponent, DockAtEntity, ExchangeWares, HarvestGas, MineAsteroid, MoveToEntity,
    RequestAccess, Undock, UseGate,
};
use crate::simulation::ship_ai::{behaviors, stop_idle_ships, tasks};
use crate::states::SimulationState;
use bevy::app::App;
use bevy::ecs::schedule::SystemConfigs;
use bevy::log::error;
use bevy::prelude::{in_state, on_event, Commands, EventReader, FixedPostUpdate, FixedUpdate, IntoSystemConfigs, IntoSystemSet, Plugin, Query, Res, With};

pub struct ShipAiPlugin;
impl Plugin for ShipAiPlugin {
    #[rustfmt::skip]
    fn build(&self, app: &mut App) {
        register_behavior(app, behaviors::auto_construct::handle_idle_ships);
        register_behavior(app, behaviors::auto_trade::handle_idle_ships);
        register_behavior(app, behaviors::auto_harvest::handle_idle_ships);
        register_behavior(app, behaviors::auto_mine::handle_idle_ships.before(asteroids::respawn_asteroids));

        register_task::<Undock, _, _, _>(app, Some(Undock::on_task_started), Undock::run_tasks, None::<Nothing>);
        register_task::<ExchangeWares, _, _, _>(app, Some(ExchangeWares::on_task_started), ExchangeWares::run_tasks, Some(ExchangeWares::complete_tasks));
        register_task::<UseGate, _, _, _>(app, Some(UseGate::on_task_started), UseGate::run_tasks, Some(UseGate::complete_tasks));
        register_task::<ConstructTaskComponent, _, _, _>(app, Some(ConstructTaskComponent::on_task_started), ConstructTaskComponent::run_tasks, None::<Nothing>);

        register_task::<MoveToEntity, _, _, _>(app, None::<Nothing>, MoveToEntity::run_tasks, None::<Nothing>);
        register_task::<DockAtEntity, _, _, _>(app, None::<Nothing>, DockAtEntity::run_tasks, Some(DockAtEntity::complete_tasks));
        register_task::<MineAsteroid, _, _, _>(app, None::<Nothing>, MineAsteroid::run_tasks, None::<Nothing>);
        register_task::<HarvestGas, _, _, _>(app, None::<Nothing>, HarvestGas::run_tasks, Some(HarvestGas::complete_tasks));

        // Unique stuff
        app.add_event::<TaskCompletedEvent<AwaitingSignal>>();
        app.add_systems(
            FixedUpdate,
            (
                stop_idle_ships::stop_idle_ships,
                
                RequestAccess::run_tasks,
                complete_tasks::<AwaitingSignal>.run_if(on_event::<TaskCompletedEvent<AwaitingSignal>>)
                    .after(complete_tasks::<Undock>)
                    .after(complete_tasks::<HarvestGas>) // TODO: Could be replaced with a more general "disengage orbit" task or something alike
                ,
            )
                .run_if(in_state(SimulationState::Running)),
        );
    }
}

/// Empty IntoSystemConfigs impl to make compiler happy
struct Nothing;
impl IntoSystemConfigs<u8> for Nothing {
    fn into_configs(self) -> SystemConfigs {
        panic!("This should never happen!")
    }
}

fn register_behavior<T>(app: &mut App, system: impl IntoSystemConfigs<T>) {
    app.add_systems(
        FixedUpdate,
        system.run_if(in_state(SimulationState::Running)),
    );
}

fn register_task<T, T1, T2, T3>(
    app: &mut App,
    on_started: Option<impl IntoSystemConfigs<T1>>,
    run: impl IntoSystemConfigs<T2> + IntoSystemSet<T2> + Copy,
    custom_on_complete: Option<impl IntoSystemConfigs<T3>>,
) where
    T: TaskComponent,
{
    app.add_event::<TaskCompletedEvent<T>>();

    if let Some(on_started) = on_started {
        app.add_event::<TaskStartedEvent<T>>();
        app.add_systems(
            FixedPostUpdate,
            on_started.run_if(in_state(SimulationState::Running)),
        );
    }

    if let Some(custom_on_complete) = custom_on_complete {
        app.add_systems(
            FixedUpdate,
            (
                run,
                (custom_on_complete, complete_tasks::<T>)
                    .run_if(on_event::<TaskCompletedEvent<T>>)
                    .chain()
            )
                .chain()
                .run_if(in_state(SimulationState::Running)));
    } else {
        app.add_systems(
            FixedUpdate,
            (
                run,
                complete_tasks::<T>.run_if(on_event::<TaskCompletedEvent<T>>),
            )
                .chain()
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
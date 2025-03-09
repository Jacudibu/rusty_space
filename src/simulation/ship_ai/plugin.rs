use crate::simulation::asteroids;
use crate::simulation::ship_ai::task_finished_event::TaskFinishedEvent;
use crate::simulation::ship_ai::task_started_event::TaskStartedEvent;
use crate::simulation::ship_ai::tasks::{
    AwaitingSignal, Construct, DockAtEntity, ExchangeWares, HarvestGas, MineAsteroid, MoveToEntity,
    RequestAccess, Undock, UseGate,
};
use crate::simulation::ship_ai::{behaviors, stop_idle_ships};
use crate::states::SimulationState;
use bevy::app::App;
use bevy::ecs::schedule::SystemConfigs;
use bevy::prelude::{
    in_state, on_event, Component, FixedPostUpdate, FixedUpdate, IntoSystemConfigs, IntoSystemSet,
    Plugin,
};

pub struct ShipAiPlugin;
impl Plugin for ShipAiPlugin {
    #[rustfmt::skip]
    fn build(&self, app: &mut App) {
        register_behavior(app, behaviors::auto_trade::handle_idle_ships);
        register_behavior(app, behaviors::auto_harvest::handle_idle_ships);
        register_behavior(app, behaviors::auto_mine::handle_idle_ships.before(asteroids::respawn_asteroids));

        register_task::<Undock, _, _, _>(app, Some(Undock::on_task_started), Undock::run_tasks, Undock::complete_tasks);
        register_task::<ExchangeWares, _, _, _>(app, Some(ExchangeWares::on_task_started), ExchangeWares::run_tasks, ExchangeWares::complete_tasks);
        register_task::<UseGate, _, _, _>(app, Some(UseGate::on_task_started), UseGate::run_tasks, UseGate::complete_tasks);
        register_task::<Construct, _, _, _>(app, Some(Construct::on_task_started), Construct::run_tasks, Construct::complete_tasks);

        register_task::<MoveToEntity, _, _, _>(app, None::<Nothing>, MoveToEntity::run_tasks, MoveToEntity::complete_tasks);
        register_task::<DockAtEntity, _, _, _>(app, None::<Nothing>, DockAtEntity::run_tasks, DockAtEntity::complete_tasks);
        register_task::<MineAsteroid, _, _, _>(app, None::<Nothing>, MineAsteroid::run_tasks, MineAsteroid::complete_tasks);
        register_task::<HarvestGas, _, _, _>(app, None::<Nothing>, HarvestGas::run_tasks, HarvestGas::complete_tasks);

        // Unique stuff
        app.add_event::<TaskFinishedEvent<AwaitingSignal>>();
        app.add_systems(
            FixedUpdate,
            (
                stop_idle_ships::stop_idle_ships,
                
                RequestAccess::run_tasks,
                AwaitingSignal::complete_tasks.run_if(on_event::<TaskFinishedEvent<AwaitingSignal>>)
                    .after(Undock::complete_tasks)
                    .after(HarvestGas::complete_tasks) // Could be replaced with a more general "disengage orbit" task or something alike
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
        todo!()
    }
}

fn register_behavior<T>(app: &mut App, system: impl IntoSystemConfigs<T>) {
    app.add_systems(
        FixedUpdate,
        system.run_if(in_state(SimulationState::Running)),
    );
}

fn register_task<TaskComponent, T1, T2, T3>(
    app: &mut App,
    on_started: Option<impl IntoSystemConfigs<T1>>,
    run: impl IntoSystemConfigs<T2> + IntoSystemSet<T2> + Copy,
    on_complete: impl IntoSystemConfigs<T3>,
) where
    TaskComponent: Component,
{
    app.add_event::<TaskFinishedEvent<TaskComponent>>();

    if let Some(on_started) = on_started {
        app.add_event::<TaskStartedEvent<TaskComponent>>();
        app.add_systems(
            FixedPostUpdate,
            on_started.run_if(in_state(SimulationState::Running)),
        );
    }

    app.add_systems(
        FixedUpdate,
        (
            run,
            on_complete
                .after(run)
                .run_if(on_event::<TaskFinishedEvent<TaskComponent>>),
        )
            .run_if(in_state(SimulationState::Running)),
    );
}

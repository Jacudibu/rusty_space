use crate::simulation::asteroids;
use crate::simulation::ship_ai::{behaviors, stop_idle_ships};
use crate::simulation::ship_ai::task_finished_event::TaskFinishedEvent;
use crate::simulation::ship_ai::tasks::{AwaitingSignal, Construct, DockAtEntity, ExchangeWares, HarvestGas, MineAsteroid, MoveToEntity, RequestAccess, Undock, UseGate};
use crate::states::SimulationState;
use bevy::app::App;
use bevy::prelude::{in_state, on_event, FixedPostUpdate, FixedUpdate, IntoSystemConfigs, Plugin};
use crate::simulation::ship_ai::task_started_event::TaskStartedEvent;

pub struct ShipAiPlugin;
impl Plugin for ShipAiPlugin {
    #[rustfmt::skip]
    fn build(&self, app: &mut App) {
        // TODO: There should be just some register function for every task which does all of this...

        app.add_event::<TaskFinishedEvent<MoveToEntity>>();
        app.add_event::<TaskFinishedEvent<DockAtEntity>>();
        app.add_event::<TaskFinishedEvent<Undock>>();
        app.add_event::<TaskFinishedEvent<ExchangeWares>>();
        app.add_event::<TaskFinishedEvent<UseGate>>();
        app.add_event::<TaskFinishedEvent<MineAsteroid>>();
        app.add_event::<TaskFinishedEvent<HarvestGas>>();
        app.add_event::<TaskFinishedEvent<AwaitingSignal>>();
        app.add_event::<TaskFinishedEvent<Construct>>();

        app.add_event::<TaskStartedEvent<ExchangeWares>>();
        app.add_event::<TaskStartedEvent<UseGate>>();
        app.add_event::<TaskStartedEvent<Undock>>();
        app.add_event::<TaskStartedEvent<Construct>>();

        app.add_systems(
            FixedUpdate,
            (
                stop_idle_ships::stop_idle_ships,
                behaviors::auto_trade::handle_idle_ships,
                behaviors::auto_mine::handle_idle_ships.before(asteroids::respawn_asteroids),
                behaviors::auto_harvest::handle_idle_ships,
                RequestAccess::run_tasks,
                AwaitingSignal::complete_tasks.run_if(on_event::<TaskFinishedEvent<AwaitingSignal>>)
                    .after(Undock::complete_tasks)  
                    .after(HarvestGas::complete_tasks) // Could be replaced with a more general "disengage orbit" task or something alike
                ,
                ExchangeWares::run_tasks,
                ExchangeWares::complete_tasks.after(ExchangeWares::run_tasks).run_if(on_event::<TaskFinishedEvent<ExchangeWares>>),
                DockAtEntity::run_tasks,
                DockAtEntity::complete_tasks.after(DockAtEntity::run_tasks).run_if(on_event::<TaskFinishedEvent<DockAtEntity>>),
                Undock::run_tasks,
                Undock::complete_tasks.after(Undock::run_tasks).run_if(on_event::<TaskFinishedEvent<Undock>>),
                MoveToEntity::run_tasks,
                MoveToEntity::complete_tasks.after(MoveToEntity::run_tasks).run_if(on_event::<TaskFinishedEvent<MoveToEntity>>),
                UseGate::run_tasks,
                UseGate::complete_tasks.after(UseGate::run_tasks).run_if(on_event::<TaskFinishedEvent<UseGate>>),
                MineAsteroid::run_tasks,
                MineAsteroid::complete_tasks.after(MineAsteroid::run_tasks).run_if(on_event::<TaskFinishedEvent<MineAsteroid>>),
                HarvestGas::run_tasks,
                HarvestGas::complete_tasks.after(HarvestGas::run_tasks).run_if(on_event::<TaskFinishedEvent<HarvestGas>>),
            )
                .run_if(in_state(SimulationState::Running)),
        );
        app.add_systems(
            FixedPostUpdate,
            (ExchangeWares::on_task_started, UseGate::on_task_started, Undock::on_task_started, Construct::on_task_creation)
                .run_if(in_state(SimulationState::Running))
        );
    }
}

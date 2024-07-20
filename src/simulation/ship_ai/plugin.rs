use crate::simulation::ship_ai::behaviors;
use crate::simulation::ship_ai::task_finished_event::TaskFinishedEvent;
use crate::simulation::ship_ai::tasks::{ExchangeWares, MineAsteroid, MoveToEntity, UseGate};
use crate::states::SimulationState;
use bevy::app::App;
use bevy::prelude::{in_state, on_event, FixedPostUpdate, FixedUpdate, IntoSystemConfigs, Plugin};

pub struct ShipAiPlugin;
impl Plugin for ShipAiPlugin {
    #[rustfmt::skip]
    fn build(&self, app: &mut App) {
        app.add_event::<TaskFinishedEvent<MoveToEntity>>();
        app.add_event::<TaskFinishedEvent<ExchangeWares>>();
        app.add_event::<TaskFinishedEvent<UseGate>>();
        app.add_event::<TaskFinishedEvent<MineAsteroid>>();
        app.add_systems(
            FixedUpdate,
            (
                behaviors::auto_trade::handle_idle_ships,
                behaviors::auto_mine::handle_idle_ships,
                ExchangeWares::run_tasks,
                ExchangeWares::complete_tasks.after(ExchangeWares::run_tasks).run_if(on_event::<TaskFinishedEvent<ExchangeWares>>()),
                MoveToEntity::run_tasks,
                MoveToEntity::complete_tasks.after(MoveToEntity::run_tasks).run_if(on_event::<TaskFinishedEvent<MoveToEntity>>()),
                UseGate::run_tasks,
                UseGate::complete_tasks.after(UseGate::run_tasks).run_if(on_event::<TaskFinishedEvent<UseGate>>()),
                MineAsteroid::run_tasks,
                MineAsteroid::complete_tasks.after(MineAsteroid::run_tasks).run_if(on_event::<TaskFinishedEvent<MineAsteroid>>()),
            )
                .run_if(in_state(SimulationState::Running)),
        );
        app.add_systems(
            FixedPostUpdate,
            (ExchangeWares::on_task_creation, UseGate::on_task_creation)
                .run_if(in_state(SimulationState::Running))
        );
    }
}

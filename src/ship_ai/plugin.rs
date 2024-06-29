use crate::ship_ai::task_finished_event::TaskFinishedEvent;
use crate::ship_ai::tasks::{ExchangeWares, Idle, MoveToEntity};
use bevy::app::App;
use bevy::prelude::{IntoSystemConfigs, Plugin, Update};

pub struct ShipAiPlugin;
impl Plugin for ShipAiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TaskFinishedEvent<MoveToEntity>>();
        app.add_event::<TaskFinishedEvent<ExchangeWares>>();
        app.add_systems(
            Update,
            (
                Idle::search_for_something_to_do,
                ExchangeWares::run_tasks,
                ExchangeWares::complete_tasks.after(ExchangeWares::run_tasks),
                MoveToEntity::run_tasks,
                MoveToEntity::complete_tasks.after(MoveToEntity::run_tasks),
            ),
        );
    }
}

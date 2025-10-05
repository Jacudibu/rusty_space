use crate::production::state::GlobalProductionState;
use crate::production::{inventory_update_event, production_runner, production_started_event};
use bevy::app::{App, Plugin};
use bevy::prelude::{FixedUpdate, IntoScheduleConfigs, in_state};
use common::events::InventoryUpdateForProductionMessage;
use common::states::SimulationState;

/// Handles everything production related.
pub struct ProductionPlugin;
impl Plugin for ProductionPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<production_started_event::ProductionStartedEvent>()
            .add_message::<InventoryUpdateForProductionMessage>()
            .insert_resource(GlobalProductionState::default())
            .add_systems(
                FixedUpdate,
                (
                    production_runner::check_if_production_is_finished_and_start_new_one,
                    production_started_event::on_production_started,
                    inventory_update_event::handle_inventory_updates,
                )
                    .run_if(in_state(SimulationState::Running)),
            );
    }
}

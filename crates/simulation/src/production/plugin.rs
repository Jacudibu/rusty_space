use crate::production::state::GlobalProductionState;
use crate::production::{inventory_update_event, production_runner, production_started_event};
use bevy::app::{App, Plugin};
use bevy::prelude::{FixedUpdate, IntoScheduleConfigs, in_state};
use common::events::inventory_update_for_production_event::InventoryUpdateForProductionEvent;
use common::states::SimulationState;

/// Handles everything production related.
pub struct ProductionPlugin;
impl Plugin for ProductionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<production_started_event::ProductionStartedEvent>()
            .add_event::<InventoryUpdateForProductionEvent>()
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

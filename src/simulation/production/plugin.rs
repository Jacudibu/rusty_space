use crate::simulation::production::state::GlobalProductionState;
use crate::simulation::production::{
    inventory_update_event, production_runner, production_started_event,
};
use bevy::app::{App, Plugin};
use bevy::prelude::FixedUpdate;

/// Handles everything production related.
pub struct ProductionPlugin;
impl Plugin for ProductionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<production_started_event::ProductionStartedEvent>()
            .add_event::<inventory_update_event::InventoryUpdateForProductionEvent>()
            .insert_resource(GlobalProductionState::default())
            .add_systems(FixedUpdate, (
                production_runner::check_if_production_is_finished_and_start_new_one,
                production_started_event::on_production_started,
                inventory_update_event::handle_inventory_updates,
            ))
        // .
        ;
    }
}

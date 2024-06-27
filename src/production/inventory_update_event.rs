use crate::components::{BuyOrders, Inventory, SellOrders};
use crate::game_data::GameData;
use crate::production::production_started_event::ProductionStartedEvent;
use crate::production::{production_runner, ProductionComponent};
use crate::simulation_time::SimulationTime;
use crate::utils;
use bevy::prelude::{Entity, Event, EventReader, EventWriter, Query, Res};

/// This event should be sent whenever an entity's inventory is being updated outside the production manager
///
/// More performant than querying with Changed<Inventory> since bevy won't need to iterate
/// through all entities matching the query every frame, plus it won't trigger itself recursively
/// ...the only risk is that we may forget to send it on inventory changes. What could go wrong?
#[derive(Event)]
pub struct InventoryUpdateForProductionEvent {
    entity: Entity,
}

impl InventoryUpdateForProductionEvent {
    pub fn new(entity: Entity) -> Self {
        Self { entity }
    }
}

pub fn handle_inventory_updates(
    simulation_time: Res<SimulationTime>,
    game_data: Res<GameData>,
    mut event_reader: EventReader<InventoryUpdateForProductionEvent>,
    mut production_start_event_writer: EventWriter<ProductionStartedEvent>,
    mut query: Query<(
        &mut ProductionComponent,
        &mut Inventory,
        Option<&mut BuyOrders>,
        Option<&mut SellOrders>,
    )>,
) {
    let current = simulation_time.seconds();
    for event in event_reader.read() {
        let Ok((mut production, mut inventory, buy_orders, sell_orders)) =
            query.get_mut(event.entity)
        else {
            continue;
        };

        for (id, module) in production.modules.iter_mut() {
            if module.current_run_finished_at.is_some() {
                continue;
            }

            let recipe = game_data.item_recipes.get(&module.recipe).unwrap();
            if inventory.has_enough_items_to_start_production(recipe, module.amount) {
                production_runner::start_production(
                    &mut production_start_event_writer,
                    current,
                    event.entity,
                    *id,
                    module,
                    &mut inventory,
                    recipe,
                );
            }
        }

        utils::update_orders(&inventory, buy_orders, sell_orders);
    }
}

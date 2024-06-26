use crate::components::{BuyOrders, Inventory, ProductionModule, SellOrders};
use crate::data::{GameData, ItemRecipe};
use crate::production::production_started_event::ProductionStartedEvent;
use crate::production::state::GlobalProductionState;
use crate::simulation_time::{SimulationSeconds, SimulationTime};
use crate::utils;
use bevy::log::error;
use bevy::prelude::{Entity, EventWriter, Mut, Query, Res, ResMut};

pub fn check_if_production_is_finished_and_start_new_one(
    simulation_time: Res<SimulationTime>,
    mut global_production_state: ResMut<GlobalProductionState>,
    game_data: Res<GameData>,
    mut production_start_event_writer: EventWriter<ProductionStartedEvent>,
    mut query: Query<(
        &mut ProductionModule,
        &mut Inventory,
        Option<&mut BuyOrders>,
        Option<&mut SellOrders>,
    )>,
) {
    let current = simulation_time.seconds();
    while let Some(next) = global_production_state.peek() {
        if current < next.finished_at {
            break;
        }

        let next = global_production_state.pop().unwrap();

        // TODO: Put this into another event?
        if let Ok((mut production, mut inventory, buy_orders, sell_orders)) =
            query.get_mut(next.entity)
        {
            let recipe = game_data.item_recipes.get(&production.recipe).unwrap();

            inventory.finish_production(recipe);
            if inventory.has_enough_items_to_start_production(recipe) {
                start_production(
                    &mut production_start_event_writer,
                    current,
                    next.entity,
                    &mut production,
                    &mut inventory,
                    recipe,
                );
            } else {
                production.current_run_finished_at = None;
            }

            utils::update_orders(&inventory, buy_orders, sell_orders);
        } else {
            error!(
                "Was unable to trigger production finish for entity {}!",
                next.entity
            );
        }
    }
}

pub(crate) fn start_production(
    production_start_event_writer: &mut EventWriter<ProductionStartedEvent>,
    current: SimulationSeconds,
    entity: Entity,
    production: &mut Mut<ProductionModule>,
    inventory: &mut Mut<Inventory>,
    recipe: &ItemRecipe,
) {
    inventory.remove_items_to_start_production(recipe);

    let finish_timestamp = current + recipe.duration;
    production.current_run_finished_at = Some(finish_timestamp);

    production_start_event_writer.send(ProductionStartedEvent::new(entity, finish_timestamp));
}

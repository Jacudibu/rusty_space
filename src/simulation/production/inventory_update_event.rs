use crate::components::{BuyOrders, Inventory, SellOrders};
use crate::game_data::{ItemManifest, RecipeElement, RecipeManifest, ShipyardModuleId};
use crate::session_data::{ShipConfigId, ShipConfigurationManifest};
use crate::simulation::prelude::{RunningProductionQueueElement, SimulationTime};
use crate::simulation::production::ProductionComponent;
use crate::simulation::production::production_kind::ProductionKind;
use crate::simulation::production::production_started_event::ProductionStartedEvent;
use crate::simulation::production::shipyard_component::{
    OngoingShipConstructionOrder, ShipyardComponent,
};
use crate::utils;
use bevy::log::error;
use bevy::prelude::{Entity, Event, EventReader, EventWriter, Or, Query, Res, With};

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

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn handle_inventory_updates(
    simulation_time: Res<SimulationTime>,
    recipes: Res<RecipeManifest>,
    ship_configs: Res<ShipConfigurationManifest>,
    mut event_reader: EventReader<InventoryUpdateForProductionEvent>,
    mut production_start_event_writer: EventWriter<ProductionStartedEvent>,
    mut query: Query<
        (
            Option<&mut ProductionComponent>,
            Option<&mut ShipyardComponent>,
            &mut Inventory,
            Option<&mut BuyOrders>,
            Option<&mut SellOrders>,
        ),
        Or<(With<ProductionComponent>, With<ShipyardComponent>)>,
    >,
    item_manifest: Res<ItemManifest>,
) {
    let now = simulation_time.now();
    for event in event_reader.read() {
        let Ok((production, shipyard, mut inventory, buy_orders, sell_orders)) =
            query.get_mut(event.entity)
        else {
            continue;
        };

        // Check Item Production Lines
        if let Some(mut production) = production {
            for (id, module) in production.modules.iter_mut() {
                if module.running_recipes.len() >= module.amount as usize {
                    continue;
                }

                let Some(next_queue_element) = module.queued_recipes.first() else {
                    continue;
                };
                let recipe = recipes
                    .get_by_ref(&next_queue_element.recipe)
                    .expect("Recipe IDs should always be correct!");
                if has_all_required_materials(&mut inventory, &recipe.input, 1)
                    && has_enough_storage_for_yields(&mut inventory, &recipe.output, &item_manifest)
                {
                    remove_recipe_items(&mut inventory, &recipe.input, 1, &item_manifest);

                    inventory.reserve_storage_space_for_production_yield(recipe);

                    let finish_timestamp = now.add_milliseconds(recipe.duration);
                    module.running_recipes.push(RunningProductionQueueElement {
                        recipe: recipe.id,
                        finished_at: finish_timestamp,
                    });

                    let queue_element = module.queued_recipes.pop().unwrap();
                    if queue_element.is_repeating {
                        module.queued_recipes.push(queue_element);
                    }

                    production_start_event_writer.send(ProductionStartedEvent::new(
                        event.entity,
                        ProductionKind::Item(*id),
                        finish_timestamp,
                    ));
                } else {
                    // TODO: Set some kind of status flag that we are missing ingredients
                }
            }
        }

        // Check Shipyard production lines
        if let Some(mut shipyard) = shipyard {
            if shipyard.queue.is_empty() {
                continue;
            }

            let mut available_module_ids: Vec<ShipyardModuleId> = shipyard
                .modules
                .iter()
                .filter_map(|(id, module)| {
                    if module.active.len() < module.amount as usize {
                        Some(*id)
                    } else {
                        None
                    }
                })
                .collect();

            if available_module_ids.is_empty() {
                continue;
            }

            let mut affordable_ships_from_queue: Vec<(usize, ShipConfigId)> = shipyard
                .queue
                .iter()
                .enumerate()
                .filter_map(|(index, config_id)| {
                    let Some(configuration) = ship_configs.get_by_id(config_id) else {
                        error!("Was unable to find a configuration with id {config_id:?}");
                        return None;
                    };

                    if has_all_required_materials(
                        &mut inventory,
                        &configuration.computed_stats.required_materials,
                        1,
                    ) {
                        Some((index, *config_id))
                    } else {
                        None
                    }
                })
                .collect();

            if affordable_ships_from_queue.is_empty() {
                continue;
            }

            // Reduce next_index if things have been popped from the queue
            let mut ships_built_this_frame = 0;
            while !affordable_ships_from_queue.is_empty() && !available_module_ids.is_empty() {
                let (next_index, next_ship_config_id) =
                    *affordable_ships_from_queue.first().unwrap();
                let next_index = next_index - ships_built_this_frame;
                let module_id = *available_module_ids.first().unwrap();
                let module = shipyard.modules.get_mut(&module_id).unwrap();

                let Some(configuration) = ship_configs.get_by_id(&next_ship_config_id) else {
                    error!(
                        "Was unable to find a configuration with id {next_ship_config_id:?}?! This should seriously never happen at this point."
                    );
                    continue;
                };

                let finish_timestamp =
                    now.add_milliseconds(configuration.computed_stats.build_time);
                module.active.push(OngoingShipConstructionOrder {
                    ship_config: next_ship_config_id,
                    finished_at: finish_timestamp,
                });

                if module.active.len() >= module.amount as usize {
                    available_module_ids.retain(|x| x != &module_id)
                }

                remove_recipe_items(
                    &mut inventory,
                    &configuration.computed_stats.required_materials,
                    1,
                    &item_manifest,
                );
                affordable_ships_from_queue.retain(|(index, config)| {
                    if index == &next_index {
                        return false;
                    }

                    let config = ship_configs.get_by_id(config).unwrap();
                    has_all_required_materials(
                        &mut inventory,
                        &config.computed_stats.required_materials,
                        1,
                    )
                });
                shipyard.queue.remove(next_index);
                ships_built_this_frame += 1;

                production_start_event_writer.send(ProductionStartedEvent::new(
                    event.entity,
                    ProductionKind::Shipyard(module_id),
                    finish_timestamp,
                ));
            }
        }

        utils::update_orders(&inventory, buy_orders, sell_orders, &item_manifest);
    }
}

/// Tests if there are enough items in stock to start a production run
pub fn has_all_required_materials(
    inventory: &mut Inventory,
    input: &Vec<RecipeElement>,
    multiplier: u32,
) -> bool {
    for element in input {
        let Some(inventory_element) = inventory.get(&element.item_id) else {
            return false;
        };

        if inventory_element.current - inventory_element.planned_selling
            < element.amount * multiplier
        {
            return false;
        }
    }

    true
}

/// Tests if there's enough storage space available to store all production yields.
fn has_enough_storage_for_yields(
    inventory: &mut Inventory,
    output: &Vec<RecipeElement>,
    item_manifest: &ItemManifest,
) -> bool {
    for element in output {
        let inventory_element = inventory.get(&element.item_id).unwrap();
        if inventory_element.reserved_production <= inventory_element.current {
            return false;
        }

        if inventory_element.reserved_production - inventory_element.current
            <= element.amount * item_manifest[element.item_id].size
        {
            return false;
        }
    }

    true
}

/// Removes all ingredients for a recipe from this inventory
fn remove_recipe_items(
    inventory: &mut Inventory,
    items: &Vec<RecipeElement>,
    multiplier: u32,
    item_manifest: &ItemManifest,
) {
    for recipe in items {
        inventory.remove_item(recipe.item_id, recipe.amount * multiplier, item_manifest);
    }
}

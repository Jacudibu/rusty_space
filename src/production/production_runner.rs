use bevy::log::error;
use bevy::prelude::{Commands, EventWriter, Or, Query, Res, ResMut, Transform, With};

use crate::components::{BuyOrders, InSector, Inventory, Sector, SellOrders};
use crate::game_data::GameData;
use crate::persistence::ShipIdMap;
use crate::production::production_kind::ProductionKind;
use crate::production::shipyard_component::ShipyardComponent;
use crate::production::state::GlobalProductionState;
use crate::production::{InventoryUpdateForProductionEvent, ProductionComponent};
use crate::session_data::SessionData;
use crate::ship_ai::BehaviorBuilder;
use crate::utils::{spawn_helpers, SimulationTime, SimulationTimestamp};
use crate::{utils, SpriteHandles};

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn check_if_production_is_finished_and_start_new_one(
    mut commands: Commands,
    sprites: Res<SpriteHandles>,
    mut sector_query: Query<&mut Sector>,
    mut ship_id_map: ResMut<ShipIdMap>,
    simulation_time: Res<SimulationTime>,
    mut global_production_state: ResMut<GlobalProductionState>,
    game_data: Res<GameData>,
    session_data: Res<SessionData>,
    mut inventory_update_writer: EventWriter<InventoryUpdateForProductionEvent>,
    mut query: Query<
        (
            Option<&mut ProductionComponent>,
            Option<&mut ShipyardComponent>,
            &mut Inventory,
            Option<&mut BuyOrders>,
            Option<&mut SellOrders>,
            &Transform,
            &InSector,
        ),
        Or<(With<ProductionComponent>, With<ShipyardComponent>)>,
    >,
) {
    let now = simulation_time.now();
    while let Some(next) = global_production_state.peek() {
        if now.has_not_passed(next.finished_at) {
            break;
        }

        let next = global_production_state.pop().unwrap();

        // TODO: Put this into another event?
        let Ok((
            production,
            shipyard,
            mut inventory,
            buy_orders,
            sell_orders,
            transform,
            in_sector,
        )) = query.get_mut(next.entity)
        else {
            error!(
                "Was unable to trigger production finish for entity {}!",
                next.entity
            );

            continue;
        };

        match next.kind {
            ProductionKind::Item(module_id) => {
                let Some(mut production) = production else {
                    error!("Was unable to find ProductionComponent for entity {} to trigger production completion!", next.entity);
                    continue;
                };

                let Some(module) = production.modules.get_mut(&module_id) else {
                    error!(
                        "Was unable to trigger production finish for entity {} and module id {}!",
                        next.entity, module_id
                    );
                    continue;
                };

                let recipe = game_data.item_recipes.get(&module.recipe).unwrap();
                inventory.finish_production(recipe, module.amount);
                module.current_run_finished_at = None;
            }
            ProductionKind::Shipyard(module_id) => {
                let Some(mut shipyard) = shipyard else {
                    error!(
                        "Was unable to find Shipyard for entity {} to trigger ship completion!",
                        next.entity
                    );
                    continue;
                };
                let Some(module) = shipyard.modules.get_mut(&module_id) else {
                    error!("Was unable to trigger shipyard construction finish for entity {} and module id {}! Guess it was destroyed?", next.entity, module_id);
                    continue;
                };

                let position = module
                    .active
                    .iter()
                    .position(|x| now.has_passed(x.finished_at))
                    .unwrap();
                let order = module.active.remove(position);

                let definition = session_data
                    .ship_configurations
                    .get(&order.ship_config)
                    .unwrap();

                spawn_helpers::spawn_ship(
                    &mut commands,
                    &sprites,
                    definition.name.clone(),
                    &mut sector_query,
                    in_sector.get(),
                    transform.translation.truncate(),
                    0.0,
                    BehaviorBuilder::AutoTrade {
                        next_idle_update: SimulationTimestamp::MIN,
                    },
                    &mut ship_id_map,
                );
            }
        }

        inventory_update_writer.send(InventoryUpdateForProductionEvent::new(next.entity));
        utils::update_orders(&inventory, buy_orders, sell_orders);
    }
}

use bevy::log::error;
use bevy::prelude::{Commands, EventWriter, Mut, Or, Query, Res, ResMut, Transform, With};

use crate::components::{BuyOrders, InSector, Inventory, Sector, SellOrders};
use crate::game_data::{ProductionModuleId, RecipeManifest, ShipyardModuleId};
use crate::persistence::{PersistentShipId, ShipIdMap};
use crate::session_data::{SessionData, ShipConfiguration, ShipConfigurationManifest};
use crate::simulation::prelude::{CurrentSimulationTimestamp, SimulationTime, SimulationTimestamp};
use crate::simulation::production::production_kind::ProductionKind;
use crate::simulation::production::shipyard_component::ShipyardComponent;
use crate::simulation::production::state::{GlobalProductionState, SingleProductionState};
use crate::simulation::production::{InventoryUpdateForProductionEvent, ProductionComponent};
use crate::simulation::ship_ai::BehaviorBuilder;
use crate::utils::spawn_helpers;
use crate::{utils, SpriteHandles};

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn check_if_production_is_finished_and_start_new_one(
    mut commands: Commands,
    sprites: Res<SpriteHandles>,
    mut sector_query: Query<&mut Sector>,
    mut ship_id_map: ResMut<ShipIdMap>,
    simulation_time: Res<SimulationTime>,
    mut global_production_state: ResMut<GlobalProductionState>,
    recipes: Res<RecipeManifest>,
    ship_configs: Res<ShipConfigurationManifest>,
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
            ProductionKind::Item(module_id) => process_finished_item_production(
                &recipes,
                &next,
                production,
                &mut inventory,
                &module_id,
            ),
            ProductionKind::Shipyard(module_id) => process_finished_ship_production(
                &mut commands,
                &sprites,
                &mut sector_query,
                &mut ship_id_map,
                &ship_configs,
                now,
                &next,
                shipyard,
                transform,
                in_sector,
                &module_id,
            ),
        }

        inventory_update_writer.send(InventoryUpdateForProductionEvent::new(next.entity));
        utils::update_orders(&inventory, buy_orders, sell_orders);
    }
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn process_finished_ship_production(
    commands: &mut Commands,
    sprites: &SpriteHandles,
    sector_query: &mut Query<&mut Sector>,
    ship_id_map: &mut ResMut<ShipIdMap>,
    ship_configs: &ShipConfigurationManifest,
    now: CurrentSimulationTimestamp,
    next: &SingleProductionState,
    shipyard: Option<Mut<ShipyardComponent>>,
    transform: &Transform,
    in_sector: &InSector,
    module_id: &ShipyardModuleId,
) {
    let Some(mut shipyard) = shipyard else {
        error!(
            "Was unable to find Shipyard for entity {} to trigger ship completion!",
            next.entity
        );
        return;
    };

    let Some(module) = shipyard.modules.get_mut(module_id) else {
        error!("Was unable to trigger shipyard construction finish for entity {} and module id {:?}! Guess it was destroyed?", next.entity, module_id);
        return;
    };

    let position = module
        .active
        .iter()
        .position(|x| now.has_passed(x.finished_at))
        .unwrap();
    let order = module.active.remove(position);

    let definition = ship_configs.get_by_id(&order.ship_config).unwrap();

    spawn_helpers::spawn_ship(
        commands,
        sprites,
        PersistentShipId::next(),
        definition.name.clone(),
        sector_query,
        in_sector.get(),
        transform.translation.truncate(),
        0.0,
        &BehaviorBuilder::AutoTrade {
            next_idle_update: SimulationTimestamp::MIN,
        },
        ship_id_map,
    );
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn process_finished_item_production(
    recipes: &RecipeManifest,
    next: &SingleProductionState,
    production: Option<Mut<ProductionComponent>>,
    inventory: &mut Inventory,
    module_id: &ProductionModuleId,
) {
    let Some(mut production) = production else {
        error!("Was unable to find ProductionComponent for entity {} to trigger production completion!", next.entity);
        return;
    };

    let Some(module) = production.modules.get_mut(module_id) else {
        error!(
            "Was unable to trigger production finish for entity {} and module id {:?}!",
            next.entity, module_id
        );
        return;
    };

    let recipe = recipes.get_by_ref(&module.recipe).unwrap();
    inventory.finish_production(recipe, module.amount);
    module.current_run_finished_at = None;
}

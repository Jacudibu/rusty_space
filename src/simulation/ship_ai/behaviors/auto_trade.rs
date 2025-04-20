use bevy::prelude::{Commands, Component, Entity, Query, Res};

use crate::components::{
    BuyOrders, InSector, InventoryComponent, SectorComponent, SellOrders, TradeOrder,
};
use crate::constants;
use crate::game_data::ItemManifest;
use crate::simulation::prelude::{SimulationTime, SimulationTimestamp};
use crate::simulation::ship_ai::TaskQueue;
use crate::simulation::ship_ai::ship_is_idle_filter::ShipIsIdleFilter;
use crate::simulation::ship_ai::task_events::AllTaskStartedEventWriters;
use crate::simulation::transform::simulation_transform::SimulationTransform;
use crate::trade_plan::TradePlan;
use crate::utils::{TradeIntent, TypedEntity};

#[derive(Component)]
pub struct AutoTradeBehavior {
    pub next_idle_update: SimulationTimestamp,
}

impl Default for AutoTradeBehavior {
    fn default() -> Self {
        Self {
            next_idle_update: SimulationTimestamp::MIN,
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn handle_idle_ships(
    mut commands: Commands,
    simulation_time: Res<SimulationTime>,
    mut ships: Query<(Entity, &mut TaskQueue, &mut AutoTradeBehavior, &InSector), ShipIsIdleFilter>,
    mut buy_orders: Query<(Entity, &mut BuyOrders, &InSector)>,
    mut sell_orders: Query<(Entity, &mut SellOrders, &InSector)>,
    mut inventories: Query<&mut InventoryComponent>,
    all_sectors: Query<&SectorComponent>,
    all_transforms: Query<&SimulationTransform>,
    item_manifest: Res<ItemManifest>,
    mut all_task_started_event_writers: AllTaskStartedEventWriters,
) {
    let now = simulation_time.now();

    // TODO: We could collect the best deals for each system in between simulation ticks and then just work on that list
    //       Maintaining it between ticks might be inefficient since production changes will shift everything around

    ships
        .iter_mut()
        .filter(|(_, _, behavior, _)| now.has_passed(behavior.next_idle_update))
        .for_each(|(ship_entity, mut queue, mut behavior, ship_sector)| {
            let inventory = inventories.get(ship_entity).unwrap();
            let plan = TradePlan::search_for_trade_run(
                inventory,
                &buy_orders,
                &sell_orders,
                &item_manifest,
            );
            let Some(plan) = plan else {
                behavior.next_idle_update =
                    now.add_seconds(constants::SECONDS_BETWEEN_SHIP_BEHAVIOR_IDLE_UPDATES);
                return;
            };
            let [
                mut this_inventory,
                mut seller_inventory,
                mut buyer_inventory,
            ] = inventories
                .get_many_mut([ship_entity, plan.seller.into(), plan.buyer.into()])
                .unwrap();

            this_inventory.create_order(
                plan.item_id,
                TradeIntent::Buy,
                plan.amount,
                &item_manifest,
            );
            seller_inventory.create_order(
                plan.item_id,
                TradeIntent::Sell,
                plan.amount,
                &item_manifest,
            );

            this_inventory.create_order(
                plan.item_id,
                TradeIntent::Sell,
                plan.amount,
                &item_manifest,
            );
            buyer_inventory.create_order(
                plan.item_id,
                TradeIntent::Buy,
                plan.amount,
                &item_manifest,
            );

            update_buy_and_sell_orders_for_entity(
                TypedEntity::Ship(ship_entity.into()),
                &this_inventory,
                &mut buy_orders,
                &mut sell_orders,
                &item_manifest,
            );
            update_buy_and_sell_orders_for_entity(
                plan.buyer,
                &buyer_inventory,
                &mut buy_orders,
                &mut sell_orders,
                &item_manifest,
            );
            update_buy_and_sell_orders_for_entity(
                plan.seller,
                &seller_inventory,
                &mut buy_orders,
                &mut sell_orders,
                &item_manifest,
            );

            plan.create_tasks_for_purchase(
                &all_sectors,
                &all_transforms,
                ship_entity,
                ship_sector,
                &mut queue,
            );

            plan.create_tasks_for_sale(&all_sectors, &all_transforms, &mut queue);
            queue.apply(
                &mut commands,
                now,
                ship_entity,
                &mut all_task_started_event_writers,
            );
        });
}

fn update_buy_and_sell_orders_for_entity(
    entity: TypedEntity,
    inventory: &InventoryComponent,
    buy_orders: &mut Query<(Entity, &mut BuyOrders, &InSector)>,
    sell_orders: &mut Query<(Entity, &mut SellOrders, &InSector)>,
    item_manifest: &ItemManifest,
) {
    if let Ok(mut buy_orders) = buy_orders.get_mut(entity.into()) {
        buy_orders.1.update(inventory, item_manifest);
    }
    if let Ok(mut sell_orders) = sell_orders.get_mut(entity.into()) {
        sell_orders.1.update(inventory, item_manifest);
    }
}

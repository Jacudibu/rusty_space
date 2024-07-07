use bevy::prelude::{Commands, Component, Entity, Query, Res, Transform};

use crate::components::{BuyOrders, InSector, Inventory, Sector, SellOrders, TradeOrder};
use crate::gizmos::find_path;
use crate::ship_ai::ship_is_idle_filter::ShipIsIdleFilter;
use crate::ship_ai::{TaskInsideQueue, TaskQueue};
use crate::trade_plan::TradePlan;
use crate::utils::{ExchangeWareData, SimulationTime, SimulationTimestamp, TradeIntent};

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
    mut ships: Query<(Entity, &mut AutoTradeBehavior, &InSector), ShipIsIdleFilter>,
    mut buy_orders: Query<(Entity, &mut BuyOrders, &InSector)>,
    mut sell_orders: Query<(Entity, &mut SellOrders, &InSector)>,
    mut inventories: Query<&mut Inventory>,
    all_sectors: Query<&Sector>,
    all_transforms: Query<&Transform>,
) {
    let now = simulation_time.now();

    ships
        .iter_mut()
        .filter(|(_, behavior, _)| now.has_passed(behavior.next_idle_update))
        .for_each(|(ship_entity, mut behavior, ship_sector)| {
            let inventory = inventories.get(ship_entity).unwrap();
            let plan =
                TradePlan::search_for_trade_run(inventory.capacity, &buy_orders, &sell_orders);
            let Some(plan) = plan else {
                behavior.next_idle_update = now.add_seconds(2);
                return;
            };
            let [mut this_inventory, mut seller_inventory, mut buyer_inventory] = inventories
                .get_many_mut([ship_entity, plan.seller, plan.buyer])
                .unwrap();

            this_inventory.create_order(plan.item_id, TradeIntent::Buy, plan.amount);
            seller_inventory.create_order(plan.item_id, TradeIntent::Sell, plan.amount);

            this_inventory.create_order(plan.item_id, TradeIntent::Sell, plan.amount);
            buyer_inventory.create_order(plan.item_id, TradeIntent::Buy, plan.amount);

            update_buy_and_sell_orders_for_entity(
                ship_entity,
                &this_inventory,
                &mut buy_orders,
                &mut sell_orders,
            );
            update_buy_and_sell_orders_for_entity(
                plan.buyer,
                &buyer_inventory,
                &mut buy_orders,
                &mut sell_orders,
            );
            update_buy_and_sell_orders_for_entity(
                plan.seller,
                &seller_inventory,
                &mut buy_orders,
                &mut sell_orders,
            );

            let mut queue = TaskQueue::with_capacity(4);

            plan.create_tasks_for_purchase(
                &all_sectors,
                &all_transforms,
                ship_entity,
                ship_sector,
                &mut queue,
            );

            plan.create_tasks_for_sale(&all_sectors, &all_transforms, &mut queue);
            queue.apply(&mut commands, now, ship_entity);
        });
}

fn update_buy_and_sell_orders_for_entity(
    entity: Entity,
    inventory: &Inventory,
    buy_orders: &mut Query<(Entity, &mut BuyOrders, &InSector)>,
    sell_orders: &mut Query<(Entity, &mut SellOrders, &InSector)>,
) {
    if let Ok(mut buy_orders) = buy_orders.get_mut(entity) {
        buy_orders.1.update(inventory);
    }
    if let Ok(mut sell_orders) = sell_orders.get_mut(entity) {
        sell_orders.1.update(inventory);
    }
}

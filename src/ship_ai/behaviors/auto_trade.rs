use crate::components::{BuyOrders, Inventory, SellOrders, TradeOrder};
use crate::ship_ai::{Idle, TaskInsideQueue, TaskQueue};
use crate::trade_plan::TradePlan;
use crate::utils::{ExchangeWareData, SimulationTime, TradeIntent};
use bevy::prelude::{Commands, Component, Entity, Query, Res};
use std::collections::VecDeque;

#[derive(Component)]
pub struct AutoTradeBehavior;

pub fn handle_idle_ships(
    mut commands: Commands,
    simulation_time: Res<SimulationTime>,
    mut ships: Query<(Entity, &AutoTradeBehavior, &mut Idle)>,
    mut buy_orders: Query<(Entity, &mut BuyOrders)>,
    mut sell_orders: Query<(Entity, &mut SellOrders)>,
    mut inventories: Query<&mut Inventory>,
) {
    let now = simulation_time.now();

    ships
        .iter_mut()
        .filter(|(_, _, task)| now.has_passed(task.next_update))
        .for_each(|(entity, _behavior, mut task)| {
            let inventory = inventories.get(entity).unwrap();
            let plan = TradePlan::create_from(inventory.capacity, &buy_orders, &sell_orders);
            let Some(plan) = plan else {
                task.next_update = now.add_seconds(2);
                return;
            };
            let [mut this_inventory, mut seller_inventory, mut buyer_inventory] = inventories
                .get_many_mut([entity, plan.seller, plan.buyer])
                .unwrap();

            this_inventory.create_order(plan.item_id, TradeIntent::Buy, plan.amount);
            seller_inventory.create_order(plan.item_id, TradeIntent::Sell, plan.amount);

            this_inventory.create_order(plan.item_id, TradeIntent::Sell, plan.amount);
            buyer_inventory.create_order(plan.item_id, TradeIntent::Buy, plan.amount);

            update_buy_and_sell_orders_for_entity(
                entity,
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

            let queue = TaskQueue {
                queue: VecDeque::from(vec![
                    TaskInsideQueue::MoveToEntity {
                        target: plan.seller,
                    },
                    TaskInsideQueue::ExchangeWares {
                        target: plan.seller,
                        data: ExchangeWareData::Buy(plan.item_id, plan.amount),
                    },
                    TaskInsideQueue::MoveToEntity { target: plan.buyer },
                    TaskInsideQueue::ExchangeWares {
                        target: plan.buyer,
                        data: ExchangeWareData::Sell(plan.item_id, plan.amount),
                    },
                ]),
            };

            let mut commands = commands.entity(entity);
            commands.remove::<Idle>();
            queue[0].create_and_insert_component(&mut commands);
            commands.insert(queue);
        });
}

fn update_buy_and_sell_orders_for_entity(
    entity: Entity,
    inventory: &Inventory,
    buy_orders: &mut Query<(Entity, &mut BuyOrders)>,
    sell_orders: &mut Query<(Entity, &mut SellOrders)>,
) {
    if let Ok(mut buy_orders) = buy_orders.get_mut(entity) {
        buy_orders.1.update(inventory);
    }
    if let Ok(mut sell_orders) = sell_orders.get_mut(entity) {
        sell_orders.1.update(inventory);
    }
}

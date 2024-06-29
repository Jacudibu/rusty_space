use crate::components::{BuyOrders, Inventory, SellOrders, ShipBehavior, TradeOrder};
use crate::ship_ai::task_inside_queue::TaskInsideQueue;
use crate::ship_ai::task_queue::TaskQueue;
use crate::trade_plan::TradePlan;
use crate::utils::ExchangeWareData;
use crate::utils::TradeIntent;
use bevy::prelude::{Commands, Component, Entity, Query};
use std::collections::VecDeque;

#[derive(Component)]
pub struct Idle;

// TODO: This should be done in a separate system per ShipBehavior, similar to how tasks work now
impl Idle {
    pub fn search_for_something_to_do(
        mut commands: Commands,
        ships: Query<(Entity, &ShipBehavior, &Self)>,
        mut buy_orders: Query<(Entity, &mut BuyOrders)>,
        mut sell_orders: Query<(Entity, &mut SellOrders)>,
        mut inventories: Query<&mut Inventory>,
    ) {
        ships
            .iter()
            .for_each(|(entity, ship_behavior, task)| match ship_behavior {
                ShipBehavior::HoldPosition => {
                    // Stay idle
                }
                ShipBehavior::AutoTrade(_data) => {
                    let inventory = inventories.get(entity).unwrap();
                    let plan =
                        TradePlan::create_from(inventory.capacity, &buy_orders, &sell_orders);
                    let Some(plan) = plan else {
                        return;
                    };
                    let [mut this_inventory, mut seller_inventory, mut buyer_inventory] =
                        inventories
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
                    commands.remove::<Self>();
                    queue[0].create_and_insert_component(&mut commands);
                    commands.insert(queue);
                }
            });
    }
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
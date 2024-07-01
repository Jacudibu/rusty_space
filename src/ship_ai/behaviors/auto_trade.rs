use crate::components::{BuyOrders, Inventory, SellOrders, TradeOrder};
use crate::sectors::{find_path, AllSectors, InSector};
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
    mut ships: Query<(Entity, &AutoTradeBehavior, &mut Idle, &InSector)>,
    mut buy_orders: Query<(Entity, &mut BuyOrders, &InSector)>,
    mut sell_orders: Query<(Entity, &mut SellOrders, &InSector)>,
    mut inventories: Query<&mut Inventory>,
    all_sectors: Res<AllSectors>,
) {
    let now = simulation_time.now();

    ships
        .iter_mut()
        .filter(|(_, _, task, _)| now.has_passed(task.next_update))
        .for_each(|(entity, _behavior, mut task, ship_sector)| {
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

            let mut queue = TaskQueue::with_capacity(4);
            if ship_sector.sector != plan.seller_sector {
                let path = find_path(&all_sectors, ship_sector.sector, plan.seller_sector).unwrap();
                for x in path {
                    queue.push_back(TaskInsideQueue::MoveToEntity {
                        target: x.enter_gate_entity,
                    });
                    queue.push_back(TaskInsideQueue::UseGate {
                        exit_sector: x.exit_sector,
                        exit_gate: x.exit_gate,
                    })
                }
            }

            queue.push_back(TaskInsideQueue::MoveToEntity {
                target: plan.seller,
            });

            queue.push_back(TaskInsideQueue::ExchangeWares {
                target: plan.seller,
                data: ExchangeWareData::Buy(plan.item_id, plan.amount),
            });

            if plan.seller_sector != plan.buyer_sector {
                let path = find_path(&all_sectors, plan.seller_sector, plan.buyer_sector).unwrap();
                for x in path {
                    queue.push_back(TaskInsideQueue::MoveToEntity {
                        target: x.enter_gate_entity,
                    });
                    queue.push_back(TaskInsideQueue::UseGate {
                        exit_sector: x.exit_sector,
                        exit_gate: x.exit_gate,
                    })
                }
            }

            queue.push_back(TaskInsideQueue::MoveToEntity { target: plan.buyer });
            queue.push_back(TaskInsideQueue::ExchangeWares {
                target: plan.buyer,
                data: ExchangeWareData::Sell(plan.item_id, plan.amount),
            });

            let mut commands = commands.entity(entity);
            commands.remove::<Idle>();
            queue[0].create_and_insert_component(&mut commands);
            commands.insert(queue);
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

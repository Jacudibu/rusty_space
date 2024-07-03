use bevy::prelude::{Commands, Component, Entity, Query, Res, Transform};

use crate::components::{BuyOrders, InSector, Inventory, Sector, SellOrders, TradeOrder};
use crate::gizmos::find_path;
use crate::ship_ai::{Idle, TaskInsideQueue, TaskQueue};
use crate::trade_plan::TradePlan;
use crate::utils::{ExchangeWareData, SimulationTime, TradeIntent};

#[derive(Component)]
pub struct AutoTradeBehavior;

pub fn handle_idle_ships(
    mut commands: Commands,
    simulation_time: Res<SimulationTime>,
    mut ships: Query<(Entity, &AutoTradeBehavior, &mut Idle, &InSector)>,
    mut buy_orders: Query<(Entity, &mut BuyOrders, &InSector)>,
    mut sell_orders: Query<(Entity, &mut SellOrders, &InSector)>,
    mut inventories: Query<&mut Inventory>,
    all_sectors: Query<&Sector>,
    all_transforms: Query<&Transform>,
) {
    let now = simulation_time.now();

    ships
        .iter_mut()
        .filter(|(_, _, task, _)| now.has_passed(task.next_update))
        .for_each(|(ship_entity, _behavior, mut task, ship_sector)| {
            let inventory = inventories.get(ship_entity).unwrap();
            let plan = TradePlan::create_from(inventory.capacity, &buy_orders, &sell_orders);
            let Some(plan) = plan else {
                task.next_update = now.add_seconds(2);
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
            if ship_sector != plan.seller_sector {
                let ship_pos = all_transforms.get(ship_entity).unwrap().translation;
                let path = find_path(
                    &all_sectors,
                    &all_transforms,
                    ship_sector.get(),
                    ship_pos,
                    plan.seller_sector,
                )
                .unwrap();
                for x in path {
                    queue.push_back(TaskInsideQueue::MoveToEntity {
                        target: x.enter_gate.into(),
                    });
                    queue.push_back(TaskInsideQueue::UseGate {
                        enter_gate: x.enter_gate,
                        exit_sector: x.exit_sector,
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
                let seller_pos = all_transforms.get(plan.seller).unwrap().translation;
                let path = find_path(
                    &all_sectors,
                    &all_transforms,
                    plan.seller_sector,
                    seller_pos,
                    plan.buyer_sector,
                )
                .unwrap();

                for x in path {
                    queue.push_back(TaskInsideQueue::MoveToEntity {
                        target: x.enter_gate.into(),
                    });
                    queue.push_back(TaskInsideQueue::UseGate {
                        enter_gate: x.enter_gate,
                        exit_sector: x.exit_sector,
                    })
                }
            }

            queue.push_back(TaskInsideQueue::MoveToEntity { target: plan.buyer });
            queue.push_back(TaskInsideQueue::ExchangeWares {
                target: plan.buyer,
                data: ExchangeWareData::Sell(plan.item_id, plan.amount),
            });

            let mut commands = commands.entity(ship_entity);
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

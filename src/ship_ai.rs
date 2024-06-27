use crate::components::{
    BuyOrders, Engine, ExchangeWareData, Inventory, SellOrders, ShipBehavior, ShipTask, TaskQueue,
    Velocity,
};
use crate::game_data::ItemId;
use crate::production::InventoryUpdateForProductionEvent;
use crate::utils::TradeIntent;
use bevy::math::EulerRot;
use bevy::prelude::{
    error, Commands, Entity, Event, EventReader, EventWriter, Query, Res, Time, Transform, Without,
};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

#[derive(Event, Copy, Clone)]
pub struct TaskFinishedEvent {
    entity: Entity,
}

/* TODO: Contemplate whether we should use one system per task?
    Some components would still be mutually required by (almost) every system (like Velocity by anything that can move),
     so they probably wouldn't be run in parallel...
    But it would simplify the inner parallel loops and guarantee that each of them takes about the same time to execute.
    Additionally, idle ships could just be skipped entirely.
    Needs some bechmarking once we have more complex tasks.
*/
pub fn run_ship_tasks(
    time: Res<Time>,
    mut ships: Query<(Entity, &TaskQueue, &Engine, &mut Velocity)>,
    mut event_writer: EventWriter<TaskFinishedEvent>,
    all_transforms: Query<&Transform>,
) {
    let ships_with_task_completions = Arc::new(Mutex::new(Vec::<TaskFinishedEvent>::new()));

    ships
        .par_iter_mut()
        .for_each(|(entity, task_queue, engine, mut velocity)| {
            if task_queue.queue.is_empty() {
                return;
            }

            let task_completed = match task_queue.queue.front().unwrap() {
                ShipTask::MoveTo(target) => {
                    if let Ok(target_transform) = all_transforms.get(*target) {
                        let entity_transform = all_transforms.get(entity).unwrap();
                        let delta = target_transform.translation.truncate()
                            - entity_transform.translation.truncate();

                        let (_, _, own_rotation) =
                            entity_transform.rotation.to_euler(EulerRot::XYZ);
                        let own_rotation = own_rotation + std::f32::consts::FRAC_PI_2;

                        let target = delta.y.atan2(delta.x);
                        let mut angle_difference = target - own_rotation;

                        if angle_difference > std::f32::consts::PI {
                            angle_difference -= 2.0 * std::f32::consts::PI;
                        } else if angle_difference < -std::f32::consts::PI {
                            angle_difference += 2.0 * std::f32::consts::PI;
                        }

                        if angle_difference - velocity.angular > 0.0 {
                            velocity.turn_left(engine, time.delta_seconds());
                        } else {
                            velocity.turn_right(engine, time.delta_seconds());
                        }

                        let distance = delta.length();

                        if angle_difference.abs() > std::f32::consts::FRAC_PI_3 {
                            velocity.decelerate(engine, time.delta_seconds());
                        } else {
                            let distance_to_stop =
                                (velocity.forward * velocity.forward) / (2.0 * engine.deceleration);

                            if distance > distance_to_stop {
                                velocity.accelerate(engine, time.delta_seconds());
                            } else {
                                velocity.decelerate(engine, time.delta_seconds());
                            }
                        }

                        distance < 5.0
                    } else {
                        todo!()
                    }
                }
                ShipTask::ExchangeWares(_, _) => true,
                ShipTask::DoNothing => {
                    // TODO: These still need to react to their surroundings somehow, maybe?
                    velocity.decelerate(engine, time.delta_seconds());
                    false
                }
            };

            if task_completed {
                ships_with_task_completions
                    .lock()
                    .unwrap()
                    .push(TaskFinishedEvent { entity });
            }
        });

    match Arc::try_unwrap(ships_with_task_completions) {
        Ok(mutex) => {
            let batch = mutex.into_inner().unwrap();
            if !batch.is_empty() {
                event_writer.send_batch(batch);
            }
        }
        Err(_) => {
            todo!()
        }
    }
}

// TODO: Just as with ship tasks, contemplate using unique events for every relevant task completion
pub fn complete_tasks(
    mut event_reader: EventReader<TaskFinishedEvent>,
    mut query: Query<&mut TaskQueue>,
    mut commands: Commands,
    mut all_storages: Query<&mut Inventory>,
    mut event_writer: EventWriter<InventoryUpdateForProductionEvent>,
) {
    for event in event_reader.read() {
        let this = event.entity;
        if let Ok(mut task_queue) = query.get_mut(this) {
            match task_queue.queue.pop_front().unwrap() {
                ShipTask::DoNothing => {}
                ShipTask::MoveTo(_) => {}
                ShipTask::ExchangeWares(other, data) => {
                    match all_storages.get_many_mut([this, other]) {
                        Ok([mut this_inv, mut other_inv]) => {
                            match data {
                                ExchangeWareData::Buy(item_id, amount) => {
                                    this_inv.complete_order(item_id, TradeIntent::Buy, amount);
                                    other_inv.complete_order(item_id, TradeIntent::Sell, amount);
                                }
                                ExchangeWareData::Sell(item_id, amount) => {
                                    this_inv.complete_order(item_id, TradeIntent::Sell, amount);
                                    other_inv.complete_order(item_id, TradeIntent::Buy, amount);
                                }
                            }
                            event_writer.send(InventoryUpdateForProductionEvent::new(this));
                            event_writer.send(InventoryUpdateForProductionEvent::new(other));
                        }
                        Err(e) => {
                            error!(
                                "Failed to execute ware exchange between {this} and {other}: {:?}",
                                e
                            );
                            continue;
                        }
                    }
                }
            }

            if task_queue.queue.is_empty() {
                commands.entity(event.entity).remove::<TaskQueue>();
            }
        }
    }
}

pub fn handle_idle_ships(
    mut commands: Commands,
    ships: Query<(Entity, &ShipBehavior), Without<TaskQueue>>,
    mut buy_orders: Query<(Entity, &mut BuyOrders)>,
    mut sell_orders: Query<(Entity, &mut SellOrders)>,
    mut inventories: Query<&mut Inventory>,
) {
    ships
        .iter()
        .for_each(|(entity, ship_behavior)| match ship_behavior {
            ShipBehavior::HoldPosition => {
                commands.entity(entity).insert(TaskQueue {
                    queue: VecDeque::from(vec![ShipTask::DoNothing]),
                });
            }
            ShipBehavior::AutoTrade(_data) => {
                let inventory = inventories.get(entity).unwrap();
                let plan = TradePlan::create_from(inventory.capacity, &buy_orders, &sell_orders);
                if let Some(plan) = plan {
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

                    commands.entity(entity).insert(TaskQueue {
                        queue: VecDeque::from(vec![
                            ShipTask::MoveTo(plan.seller),
                            ShipTask::ExchangeWares(
                                plan.seller,
                                ExchangeWareData::Buy(plan.item_id, plan.amount),
                            ),
                            ShipTask::MoveTo(plan.buyer),
                            ShipTask::ExchangeWares(
                                plan.buyer,
                                ExchangeWareData::Sell(plan.item_id, plan.amount),
                            ),
                        ]),
                    });
                } else {
                    commands.entity(entity).insert(TaskQueue {
                        queue: VecDeque::from(vec![ShipTask::DoNothing]),
                    });
                }
            }
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

struct TradePlan {
    item_id: ItemId,
    amount: u32,
    profit: u32,
    seller: Entity,
    buyer: Entity,
}

impl TradePlan {
    pub fn create_from(
        storage_capacity: u32,
        buy_orders: &Query<(Entity, &mut BuyOrders)>,
        sell_orders: &Query<(Entity, &mut SellOrders)>,
    ) -> Option<Self> {
        let mut best_offer: Option<TradePlan> = None;

        for (buyer, buy_orders) in buy_orders.iter() {
            for (seller, sell_orders) in sell_orders.iter() {
                if buyer == seller {
                    continue;
                }

                for (item_id, buy_order) in &buy_orders.orders {
                    if let Some(sell_order) = sell_orders.orders.get(item_id) {
                        if sell_order.price >= buy_order.price {
                            continue;
                        }

                        let amount = storage_capacity.min(buy_order.amount.min(sell_order.amount));
                        if amount == 0 {
                            // TODO: Add custom definable minimum amount
                            continue;
                        }

                        let profit = (buy_order.price - sell_order.price) * amount;

                        let is_this_a_better_offer = if let Some(existing_offer) = &best_offer {
                            profit > existing_offer.profit
                        } else {
                            true
                        };

                        if is_this_a_better_offer {
                            best_offer = Some(TradePlan {
                                item_id: *item_id,
                                amount,
                                profit,
                                seller,
                                buyer,
                            });
                        }
                    }
                }
            }
        }

        best_offer
    }
}

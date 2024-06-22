use crate::components::{
    BuyOrders, Engine, ExchangeWareData, SellOrders, ShipBehavior, ShipTask, Storage, TaskQueue,
    Velocity,
};
use crate::data::{ItemId, DEBUG_ITEM_ID};
use bevy::math::EulerRot;
use bevy::prelude::{
    error, warn, Commands, Entity, Event, EventReader, EventWriter, Query, Res, Time, Transform,
    Without,
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
    mut all_storages: Query<&mut Storage>,
    mut all_buy_orders: Query<&mut BuyOrders>,
    mut all_sell_orders: Query<&mut SellOrders>,
) {
    let item_id = DEBUG_ITEM_ID;
    for event in event_reader.read() {
        if let Ok(mut task_queue) = query.get_mut(event.entity) {
            match task_queue.queue.pop_front().unwrap() {
                ShipTask::DoNothing => {}
                ShipTask::MoveTo(_) => {}
                ShipTask::ExchangeWares(other, data) => {
                    match all_storages.get_many_mut([event.entity, other]) {
                        Ok([mut this_storage, mut other_storage]) => {
                            match data {
                                ExchangeWareData::Buy(amount) => {
                                    this_storage.add_item(item_id, amount);
                                    other_storage.remove_item(item_id, amount);
                                }
                                ExchangeWareData::Sell(amount) => {
                                    this_storage.remove_item(item_id, amount);
                                    other_storage.add_item(item_id, amount);
                                }
                            }
                            if let Ok(mut buy_orders) = all_buy_orders.get_mut(other) {
                                buy_orders.update(&other_storage);
                            }
                            if let Ok(mut buy_orders) = all_buy_orders.get_mut(event.entity) {
                                buy_orders.update(&this_storage);
                            }
                            if let Ok(mut sell_orders) = all_sell_orders.get_mut(other) {
                                sell_orders.update(&other_storage);
                            }
                            if let Ok(mut sell_orders) = all_sell_orders.get_mut(event.entity) {
                                sell_orders.update(&this_storage);
                            }
                        }
                        Err(e) => {
                            error!(
                                "Failed to execute ware exchange between {} and {other}: {:?}",
                                event.entity, e
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
    ships: Query<(Entity, &ShipBehavior, &Storage), Without<TaskQueue>>,
    buy_orders: Query<(Entity, &BuyOrders)>,
    sell_orders: Query<(Entity, &SellOrders)>,
) {
    ships
        .iter()
        .for_each(|(entity, ship_behavior, storage)| match ship_behavior {
            ShipBehavior::HoldPosition => {
                commands.entity(entity).insert(TaskQueue {
                    queue: VecDeque::from(vec![ShipTask::DoNothing]),
                });
            }
            ShipBehavior::AutoTrade(data) => {
                let plan = TradePlan::create_from(storage.capacity, &buy_orders, &sell_orders);
                if let Some(plan) = plan {
                    commands.entity(entity).insert(TaskQueue {
                        queue: VecDeque::from(vec![
                            ShipTask::MoveTo(plan.seller),
                            ShipTask::ExchangeWares(
                                plan.seller,
                                ExchangeWareData::Buy(plan.amount),
                            ),
                            ShipTask::MoveTo(plan.buyer),
                            ShipTask::ExchangeWares(
                                plan.seller,
                                ExchangeWareData::Sell(plan.amount),
                            ),
                        ]),
                    });
                } else {
                    warn!("Was unable to find a trade plan for {:?}", entity);
                }
            }
        });
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
        buy_orders: &Query<(Entity, &BuyOrders)>,
        sell_orders: &Query<(Entity, &SellOrders)>,
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

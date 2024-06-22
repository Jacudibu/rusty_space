use crate::components::{
    Engine, ExchangeWareData, ShipBehavior, ShipTask, Storage, TaskQueue, TradeHub, Velocity,
};
use crate::ids::DEBUG_ITEM_ID;
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
) {
    let item_id = DEBUG_ITEM_ID;
    for event in event_reader.read() {
        if let Ok(mut task_queue) = query.get_mut(event.entity) {
            match task_queue.queue.pop_front().unwrap() {
                ShipTask::DoNothing => {}
                ShipTask::MoveTo(_) => {}
                ShipTask::ExchangeWares(other, data) => {
                    match all_storages.get_many_mut([event.entity, other]) {
                        Ok([mut this, mut other]) => match data {
                            ExchangeWareData::Buy(amount) => {
                                this.add_item(item_id, amount);
                                other.remove_item(item_id, amount);
                            }
                            ExchangeWareData::Sell(amount) => {
                                this.remove_item(item_id, amount);
                                other.add_item(item_id, amount);
                            }
                        },
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
    trade_hubs: Query<(Entity, &TradeHub, &Storage)>,
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
                // TODO: dynamically match sell & buy offers
                let best_offer_amount = 1000;
                let (seller_entity, _, _) =
                    trade_hubs.iter().find(|(_, hub, _)| hub.selling).unwrap();
                let (buyer_entity, _, _) =
                    trade_hubs.iter().find(|(_, hub, _)| hub.buying).unwrap();

                let amount = (storage.capacity - storage.used()).min(best_offer_amount);

                // TODO: Actually buy and sell stuff. Also consider reserving goods so we don't get ten ships doing the same thing.
                commands.entity(entity).insert(TaskQueue {
                    queue: VecDeque::from(vec![
                        ShipTask::MoveTo(seller_entity),
                        ShipTask::ExchangeWares(seller_entity, ExchangeWareData::Buy(amount)),
                        ShipTask::MoveTo(buyer_entity),
                        ShipTask::ExchangeWares(seller_entity, ExchangeWareData::Sell(amount)),
                    ]),
                });
            }
        });
}

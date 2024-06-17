use crate::components::{Engine, ShipBehavior, ShipTask, Storage, TaskQueue, TradeHub, Velocity};
use bevy::math::EulerRot;
use bevy::prelude::{Commands, Entity, Event, Query, Res, Time, Transform, Without};
use std::sync::{Arc, Mutex};

#[derive(Event, Copy, Clone)]
pub struct TaskFinishedEvent {
    entity: Entity,
}

pub fn run_ship_ai(
    time: Res<Time>,
    mut ships: Query<(Entity, &mut TaskQueue, &Engine, &mut Velocity)>,
    all_transforms: Query<&Transform>,
) {
    let ships_with_task_completions = Arc::new(Mutex::new(Vec::<Entity>::new()));

    ships
        .par_iter_mut()
        .for_each(|(entity, task_queue, engine, mut velocity)| {
            if task_queue.queue.is_empty() {
                return;
            }

            let task_completed = match task_queue.queue.first().unwrap() {
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
                        } else if distance > 10.0 {
                            velocity.accelerate(engine, time.delta_seconds());
                        } else {
                            velocity.decelerate(engine, time.delta_seconds());
                        }

                        distance < 5.0
                    } else {
                        todo!()
                    }
                }
                ShipTask::DoNothing => {
                    // TODO: These still need to react to their surroundings somehow.
                    false
                }
            };

            if task_completed {
                ships_with_task_completions.lock().unwrap().push(entity);
            }
        });

    for x in ships_with_task_completions.lock().unwrap().iter() {
        // TODO: Turn this into an event?
        ships.get_mut(*x).unwrap().1.queue.remove(0);
    }
}

pub fn handle_idle_ships(
    mut commands: Commands,
    ships: Query<(Entity, &ShipBehavior), Without<TaskQueue>>,
    trade_hubs: Query<(Entity, &TradeHub, &Storage)>,
) {
    ships
        .iter()
        .for_each(|(entity, ship_behavior)| match ship_behavior {
            ShipBehavior::HoldPosition => {
                commands.entity(entity).insert(TaskQueue {
                    queue: vec![ShipTask::DoNothing],
                });
            }
            ShipBehavior::AutoTrade(data) => {
                // TODO: dynamically match sell & buy offers
                let (seller_entity, _, _) =
                    trade_hubs.iter().find(|(_, hub, _)| hub.selling).unwrap();
                let (buyer_entity, _, _) =
                    trade_hubs.iter().find(|(_, hub, _)| hub.buying).unwrap();

                // TODO: Actually buy and sell stuff. Also consider reserving goods so we don't get ten ships doing the same thing.
                commands.entity(entity).insert(TaskQueue {
                    queue: vec![
                        ShipTask::MoveTo(seller_entity),
                        ShipTask::MoveTo(buyer_entity),
                    ],
                });
            }
        });
}

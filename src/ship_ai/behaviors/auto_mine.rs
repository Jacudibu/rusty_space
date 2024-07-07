use bevy::prelude::{error, Commands, Component, Entity, Query, Res, Transform};
use std::cmp::Ordering;

use crate::components::{BuyOrders, InSector, Inventory, Sector, SellOrders, TradeOrder};
use crate::ship_ai::ship_is_idle_filter::ShipIsIdleFilter;
use crate::ship_ai::{TaskInsideQueue, TaskQueue};
use crate::trade_plan::TradePlan;
use crate::utils::{SimulationTime, SimulationTimestamp, TradeIntent};

#[derive(Eq, PartialEq)]
enum AutoMineState {
    Mining,
    Trading,
}

#[derive(Component)]
pub struct AutoMineBehavior {
    pub next_idle_update: SimulationTimestamp,
    state: AutoMineState,
}

impl Default for AutoMineBehavior {
    fn default() -> Self {
        Self {
            next_idle_update: SimulationTimestamp::MIN,
            state: AutoMineState::Mining,
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn handle_idle_ships(
    mut commands: Commands,
    simulation_time: Res<SimulationTime>,
    mut ships: Query<(Entity, &mut TaskQueue, &mut AutoMineBehavior, &InSector), ShipIsIdleFilter>,
    mut buy_orders: Query<(Entity, &mut BuyOrders, &InSector)>,
    mut inventories: Query<&mut Inventory>,
    all_sectors: Query<&Sector>,
    all_transforms: Query<&Transform>,
) {
    let now = simulation_time.now();

    // TODO: Benchmark this vs a priority queue
    ships
        .iter_mut()
        .filter(|(_, _, behavior, _)| now.has_passed(behavior.next_idle_update))
        .for_each(|(ship_entity, mut queue, mut behavior, in_sector)| {
            let ship_inventory = inventories.get_mut(ship_entity).unwrap();
            let used_inventory_space = ship_inventory.used();

            if behavior.state == AutoMineState::Mining && used_inventory_space == ship_inventory.capacity
            {
                behavior.state = AutoMineState::Trading;
            } else if behavior.state == AutoMineState::Trading && used_inventory_space == 0 {
                behavior.state = AutoMineState::Mining;
            }

            match behavior.state {
                AutoMineState::Mining => {
                    let sector = all_sectors.get(in_sector.sector.into()).unwrap();
                    let ship_pos = all_transforms.get(ship_entity).unwrap().translation;
                    if let Some(asteroid_data) = sector.asteroid_data {
                        // TODO: Also Test whether asteroid_data contains the requested asteroid type

                        if let Some(closest_asteroid) = sector.asteroids.iter().min_by(|&a, &b| {
                            let a_distance = all_transforms
                                .get(a.into())
                                .unwrap()
                                .translation
                                .distance_squared(ship_pos);
                            let b_distance = all_transforms
                                .get(b.into())
                                .unwrap()
                                .translation
                                .distance_squared(ship_pos);

                            if let Some(ord) = a_distance.partial_cmp(&b_distance) {
                                ord
                            } else {
                                error!("ord was None? This should never happen, please fix. a_distance: {a_distance}, b_distance: {b_distance}");
                                Ordering::Equal
                            }
                        }) {
                            queue.push_back(TaskInsideQueue::MoveToEntity {
                                target: closest_asteroid.into(),
                            });
                            queue.push_back(TaskInsideQueue::MineAsteroid {
                                target: closest_asteroid.entity,
                            });

                            queue.apply(&mut commands, now, ship_entity);
                        } else {
                            behavior.next_idle_update.add_milliseconds(2000);
                            // TODO: No asteroids found -> Sector has been fully mined.
                            //       Either go somewhere else or just idle until a new one spawns.
                        }
                    } else {
                        // TODO: Move to a sector with asteroids in it
                    }
                }
                AutoMineState::Trading => {
                    let Some(plan) = TradePlan::sell_own_inventory(ship_entity.into(), in_sector, &ship_inventory, &buy_orders) else {
                        behavior.next_idle_update.add_milliseconds(2000);
                        return;
                    };

                    let [mut this_inventory, mut buyer_inventory] = inventories
                        .get_many_mut([ship_entity, plan.buyer])
                        .unwrap();

                    this_inventory.create_order(plan.item_id, TradeIntent::Sell, plan.amount);
                    buyer_inventory.create_order(plan.item_id, TradeIntent::Buy, plan.amount);
                    
                    plan.create_tasks_for_sale(&all_sectors, &all_transforms, &mut queue);
                    queue.apply(&mut commands, now, ship_entity);
                }
            }
        });
}

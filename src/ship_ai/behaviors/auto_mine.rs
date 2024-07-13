use bevy::math::Vec3;
use bevy::prelude::{error, Commands, Component, Entity, Query, Res, Transform};
use std::cmp::Ordering;

use crate::components::{Asteroid, BuyOrders, InSector, Inventory, Sector};
use crate::pathfinding;
use crate::persistence::SectorIdMap;
use crate::ship_ai::ship_is_idle_filter::ShipIsIdleFilter;
use crate::ship_ai::{TaskInsideQueue, TaskQueue};
use crate::trade_plan::TradePlan;
use crate::utils::{AsteroidEntityWithTimestamp, SimulationTime, SimulationTimestamp, TradeIntent};

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
    buy_orders: Query<(Entity, &mut BuyOrders, &InSector)>,
    mut inventories: Query<&mut Inventory>,
    all_sectors: Query<&Sector>,
    mut all_asteroids: Query<&mut Asteroid>,
    all_transforms: Query<&Transform>,
    sector_id_map: Res<SectorIdMap>,
) {
    let now = simulation_time.now();

    // Avoids selecting an asteroid which is close to leaving the sector
    let max_asteroid_age = now.add_milliseconds(15000);

    // TODO: Benchmark this vs a priority queue
    ships
        .iter_mut()
        .filter(|(_, _, behavior, _)| now.has_passed(behavior.next_idle_update))
        .for_each(|(ship_entity, mut queue, mut behavior, in_sector)| {
            let ship_inventory = inventories.get_mut(ship_entity).unwrap();
            let used_inventory_space = ship_inventory.used();

            if behavior.state == AutoMineState::Mining
                && used_inventory_space == ship_inventory.capacity
            {
                behavior.state = AutoMineState::Trading;
            } else if behavior.state == AutoMineState::Trading && used_inventory_space == 0 {
                behavior.state = AutoMineState::Mining;
            }

            match behavior.state {
                AutoMineState::Mining => {
                    let sector = all_sectors.get(in_sector.sector.into()).unwrap();
                    let ship_pos = all_transforms.get(ship_entity).unwrap().translation;
                    if let Some(_asteroid_data) = sector.asteroid_data {
                        // TODO: Also Test whether asteroid_data contains the requested asteroid type

                        if let Some(closest_asteroid) = sector
                            .asteroids
                            .iter()
                            .filter(|x| max_asteroid_age.has_not_passed(&x.timestamp))
                            .filter(|x| {
                                all_asteroids
                                    .get(x.entity.into())
                                    .unwrap()
                                    .remaining_after_reservations
                                    > 0
                            })
                            .min_by(|&a, &b| {
                                compare_asteroid_distances(&all_transforms, ship_pos, a, b)
                            })
                        {
                            let mut asteroid = all_asteroids
                                .get_mut(closest_asteroid.entity.into())
                                .unwrap();

                            let reserved_amount = asteroid
                                .try_to_reserve(ship_inventory.capacity - used_inventory_space);

                            queue.push_back(TaskInsideQueue::MoveToEntity {
                                target: closest_asteroid.into(),
                                stop_at_target: true,
                            });
                            queue.push_back(TaskInsideQueue::MineAsteroid {
                                target: closest_asteroid.entity,
                                reserved: reserved_amount,
                            });

                            queue.apply(&mut commands, now, ship_entity);
                        } else {
                            behavior.next_idle_update = now.add_milliseconds(2000);
                            // TODO: No asteroids found -> Sector has been fully mined.
                            //       Either go somewhere else or just idle until a new one spawns.
                        }
                    } else {
                        behavior.next_idle_update = now.add_milliseconds(2000);
                        // TODO: Properly search for nearest sector with resources
                        let target_sector = all_sectors
                            .iter()
                            .find(|x| x.asteroid_data.is_some())
                            .unwrap();
                        let path = pathfinding::find_path(
                            &all_sectors,
                            &all_transforms,
                            in_sector.sector,
                            all_transforms.get(ship_entity).unwrap().translation,
                            sector_id_map.id_to_entity()[&target_sector.coordinate],
                        )
                        .unwrap();
                        pathfinding::create_tasks_to_follow_path(&mut queue, path);
                        queue.apply(&mut commands, now, ship_entity);
                    }
                }
                AutoMineState::Trading => {
                    let Some(plan) = TradePlan::sell_anything_from_inventory(
                        ship_entity,
                        in_sector,
                        &ship_inventory,
                        &buy_orders,
                    ) else {
                        behavior.next_idle_update = now.add_milliseconds(2000);
                        return;
                    };

                    let [mut this_inventory, mut buyer_inventory] =
                        inventories.get_many_mut([ship_entity, plan.buyer]).unwrap();

                    this_inventory.create_order(plan.item_id, TradeIntent::Sell, plan.amount);
                    buyer_inventory.create_order(plan.item_id, TradeIntent::Buy, plan.amount);

                    plan.create_tasks_for_sale(&all_sectors, &all_transforms, &mut queue);
                    queue.apply(&mut commands, now, ship_entity);
                }
            }
        });
}

fn compare_asteroid_distances(
    all_transforms: &Query<&Transform>,
    ship_pos: Vec3,
    a: &AsteroidEntityWithTimestamp,
    b: &AsteroidEntityWithTimestamp,
) -> Ordering {
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
}

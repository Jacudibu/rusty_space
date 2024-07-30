use crate::components::{
    Asteroid, BuyOrders, InSector, Inventory, Sector, SectorAsteroidComponent,
};
use crate::pathfinding;
use crate::simulation::prelude::{SimulationTime, SimulationTimestamp};
use crate::simulation::ship_ai::ship_is_idle_filter::ShipIsIdleFilter;
use crate::simulation::ship_ai::{TaskInsideQueue, TaskQueue};
use crate::simulation::transform::simulation_transform::SimulationTransform;
use crate::trade_plan::TradePlan;
use crate::utils::{AsteroidEntityWithTimestamp, SectorEntity, TradeIntent};
use bevy::prelude::{error, Commands, Component, Entity, Query, Res, Vec2};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Eq, PartialEq, Serialize, Deserialize, Copy, Clone)]
#[cfg_attr(test, derive(Debug))]
pub enum AutoMineState {
    Mining,
    Trading,
}

#[derive(Component)]
pub struct AutoMineBehavior {
    pub next_idle_update: SimulationTimestamp,
    pub state: AutoMineState,
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
    all_sectors_with_asteroids: Query<&SectorAsteroidComponent>,
    all_sectors: Query<&Sector>,
    mut all_asteroids: Query<&mut Asteroid>,
    all_transforms: Query<&SimulationTransform>,
) {
    let now = simulation_time.now();

    // Avoids selecting an asteroid which is close to leaving the sector
    let max_asteroid_age = now.add_milliseconds(15000);

    // TODO: Benchmark this .filter vs a priority queue
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
                    if let Ok(asteroid_component) =
                        all_sectors_with_asteroids.get(in_sector.sector.into())
                    {
                        let ship_pos = all_transforms.get(ship_entity).unwrap().translation;

                        // TODO: Also Test whether asteroid_data contains the requested asteroid type
                        if let Some(closest_asteroid) = asteroid_component
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
                                target: closest_asteroid.entity.into(),
                                stop_at_target: true,
                            });
                            queue.push_back(TaskInsideQueue::MineAsteroid {
                                target: closest_asteroid.entity,
                                reserved: reserved_amount,
                            });

                            queue.apply(&mut commands, now, ship_entity);
                            return;
                        }
                    }

                    // No asteroids available in current sector, go somewhere else!
                    let target_sector = match find_nearby_sector_with_asteroids(
                        &all_sectors_with_asteroids,
                        &all_sectors,
                        in_sector,
                    ) {
                        Some(value) => value,
                        None => {
                            behavior.next_idle_update = now.add_milliseconds(2000);
                            return;
                        }
                    };

                    let path = pathfinding::find_path(
                        &all_sectors,
                        &all_transforms,
                        in_sector.sector,
                        all_transforms.get(ship_entity).unwrap().translation,
                        target_sector,
                        None,
                    )
                    .unwrap();
                    pathfinding::create_tasks_to_follow_path(&mut queue, path);
                    queue.apply(&mut commands, now, ship_entity);
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

                    let [mut this_inventory, mut buyer_inventory] = inventories
                        .get_many_mut([ship_entity, plan.buyer.into()])
                        .unwrap();

                    this_inventory.create_order(plan.item_id, TradeIntent::Sell, plan.amount);
                    buyer_inventory.create_order(plan.item_id, TradeIntent::Buy, plan.amount);

                    plan.create_tasks_for_sale(&all_sectors, &all_transforms, &mut queue);
                    queue.apply(&mut commands, now, ship_entity);
                }
            }
        });
}

fn find_nearby_sector_with_asteroids(
    all_sectors_with_asteroids: &Query<&SectorAsteroidComponent>,
    all_sectors: &Query<&Sector>,
    in_sector: &InSector,
) -> Option<SectorEntity> {
    let nearby_sectors_with_asteroids =
        pathfinding::surrounding_sector_search::surrounding_sector_search(
            all_sectors,
            in_sector.sector,
            1,
            u8::MAX, // TODO: Should be limited
            all_sectors_with_asteroids,
            |_| true,
        );

    let target_sector = nearby_sectors_with_asteroids.iter().min_by_key(|item| {
        let asteroid_data = all_sectors_with_asteroids.get(item.sector.into()).unwrap();

        let health = asteroid_data.remaining_percentage();

        if health > 0.4 {
            item.distance as u16
        } else {
            (item.distance * 10) as u16 * item.distance as u16
                + ((1.0 - health.powi(2)) * 100.0) as u16
        }
    })?;

    Some(target_sector.sector)
}

fn compare_asteroid_distances(
    all_transforms: &Query<&SimulationTransform>,
    ship_pos: Vec2,
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

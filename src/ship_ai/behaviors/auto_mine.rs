use bevy::prelude::{Commands, Component, Entity, Query, Res, Transform};
use std::cmp::Ordering;

use crate::components::{BuyOrders, InSector, Inventory, Sector, SellOrders, TradeOrder};
use crate::ship_ai::ship_is_idle_filter::ShipIsIdleFilter;
use crate::ship_ai::{TaskInsideQueue, TaskQueue};
use crate::utils::{SimulationTime, SimulationTimestamp};

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
    mut ships: Query<(Entity, &mut AutoMineBehavior, &InSector), ShipIsIdleFilter>,
    mut buy_orders: Query<(Entity, &mut BuyOrders, &InSector)>,
    mut inventories: Query<&mut Inventory>,
    all_sectors: Query<&Sector>,
    all_transforms: Query<&Transform>,
) {
    let now = simulation_time.now();

    // TODO: Benchmark this vs a priority queue
    ships
        .iter_mut()
        .filter(|(_, behavior, _)| now.has_passed(behavior.next_idle_update))
        .for_each(|(ship_entity, mut behavior, in_sector)| {
            behavior.next_idle_update.add_milliseconds(2000);
            let inventory = inventories.get_mut(ship_entity).unwrap();
            let used_inventory_space = inventory.used();

            if behavior.state == AutoMineState::Mining && used_inventory_space == inventory.capacity
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
                                Ordering::Equal
                            }
                        }) {
                            let mut queue = TaskQueue::with_capacity(2);
                            queue.push_back(TaskInsideQueue::MoveToEntity {
                                target: closest_asteroid.into(),
                            });
                            queue.push_back(TaskInsideQueue::MineAsteroid {
                                target: closest_asteroid.entity,
                            });

                            let mut commands = commands.entity(ship_entity);
                            queue[0].create_and_insert_component(&mut commands, now);
                            commands.insert(queue);
                        } else {
                            // TODO: No asteroids found -> Sector has been fully mined.
                            //       Either go somewhere else or just idle.
                        }
                    } else {
                        // TODO: Move to a sector with asteroids in it
                    }
                }
                AutoMineState::Trading => {
                    // TODO: Sell all items in inventory
                }
            }
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

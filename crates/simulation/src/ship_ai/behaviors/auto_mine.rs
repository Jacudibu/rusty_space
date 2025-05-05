use crate::ship_ai::create_tasks_following_path::create_tasks_to_follow_path;
use crate::ship_ai::ship_is_idle_filter::ShipIsIdleFilter;
use crate::ship_ai::tasks::apply_new_task_queue;
use crate::ship_ai::trade_plan::TradePlan;
use bevy::prelude::{Commands, Entity, Query, Res, Vec2};
use common::components::ship_behavior::ShipBehavior;
use common::components::task_queue::TaskInsideQueue;
use common::components::task_queue::TaskQueue;
use common::components::{Asteroid, BuyOrders, InSector, Inventory, Sector, SectorWithAsteroids};
use common::events::task_events::AllTaskStartedEventWriters;
use common::game_data::{ItemId, ItemManifest};
use common::simulation_time::SimulationTime;
use common::simulation_transform::SimulationTransform;
use common::types::auto_mine_state::AutoMineState;
use common::types::entity_wrappers::SectorEntity;
use common::types::ship_behaviors::AutoMineBehavior;
use common::types::trade_intent::TradeIntent;

#[allow(clippy::too_many_arguments)]
pub fn handle_idle_ships(
    mut commands: Commands,
    simulation_time: Res<SimulationTime>,
    mut ships: Query<
        (
            Entity,
            &mut TaskQueue,
            &mut ShipBehavior<AutoMineBehavior>,
            &InSector,
        ),
        ShipIsIdleFilter,
    >,
    buy_orders: Query<(Entity, &mut BuyOrders, &InSector)>,
    mut inventories: Query<&mut Inventory>,
    all_sectors_with_asteroids: Query<&SectorWithAsteroids>,
    all_sectors: Query<&Sector>,
    mut all_asteroids: Query<&mut Asteroid>,
    all_transforms: Query<&SimulationTransform>,
    item_manifest: Res<ItemManifest>,
    mut all_task_started_event_writers: AllTaskStartedEventWriters,
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
            let used_space = ship_inventory.total_used_space();
            let remaining_space =
                ship_inventory.remaining_space_for(&behavior.mined_ore, &item_manifest);

            behavior
                .state
                .flip_task_depending_on_inventory(used_space, remaining_space);

            match behavior.state {
                AutoMineState::Mining => {
                    if let Ok(asteroid_component) =
                        all_sectors_with_asteroids.get(in_sector.sector.into())
                    {
                        let ship_pos = all_transforms.get(ship_entity).unwrap().translation;

                        // TODO: Also Test whether asteroid_data contains the requested asteroid type
                        //          all_asteroids needs to be split by the item in order for this to work efficiently
                        if let Some(closest_asteroid) = asteroid_component
                            .asteroids
                            .get(&behavior.mined_ore)
                            .iter()
                            .flat_map(|x| x.iter())
                            .filter(|x| max_asteroid_age.has_not_passed(&x.timestamp))
                            .filter(|x| {
                                all_asteroids
                                    .get(x.entity.into())
                                    .unwrap()
                                    .remaining_after_reservations
                                    > 0
                            })
                            .min_by_key(|&asteroid| {
                                entity_distance_to_ship_squared(&all_transforms, ship_pos, asteroid)
                            })
                        {
                            let mut asteroid = all_asteroids
                                .get_mut(closest_asteroid.entity.into())
                                .unwrap();

                            let reserved_amount = asteroid.try_to_reserve(remaining_space);

                            queue.push_back(TaskInsideQueue::MoveToEntity {
                                target: closest_asteroid.entity.into(),
                                stop_at_target: true,
                                distance_to_target: 0.0,
                            });
                            queue.push_back(TaskInsideQueue::MineAsteroid {
                                target: closest_asteroid.entity,
                                reserved: reserved_amount,
                            });

                            apply_new_task_queue(
                                &queue,
                                &mut commands,
                                now,
                                ship_entity,
                                &mut all_task_started_event_writers,
                            );
                            return;
                        }
                    }

                    // No asteroids available in current sector, go somewhere else!
                    let target_sector = match find_nearby_sector_with_asteroids(
                        &all_sectors_with_asteroids,
                        &all_sectors,
                        in_sector,
                        &behavior.mined_ore,
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
                    create_tasks_to_follow_path(&mut queue, path);
                    apply_new_task_queue(
                        &queue,
                        &mut commands,
                        now,
                        ship_entity,
                        &mut all_task_started_event_writers,
                    );
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

                    this_inventory.create_order(
                        plan.item_id,
                        TradeIntent::Sell,
                        plan.amount,
                        &item_manifest,
                    );
                    buyer_inventory.create_order(
                        plan.item_id,
                        TradeIntent::Buy,
                        plan.amount,
                        &item_manifest,
                    );

                    plan.create_tasks_for_sale(&all_sectors, &all_transforms, &mut queue);
                    apply_new_task_queue(
                        &queue,
                        &mut commands,
                        now,
                        ship_entity,
                        &mut all_task_started_event_writers,
                    );
                }
            }
        });
}

#[must_use]
fn find_nearby_sector_with_asteroids(
    all_sectors_with_asteroids: &Query<&SectorWithAsteroids>,
    all_sectors: &Query<&Sector>,
    in_sector: &InSector,
    requested_material: &ItemId,
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

        let health = asteroid_data.remaining_percentage(requested_material);

        if health > 0.4 {
            item.distance as u16
        } else {
            (item.distance * 10) as u16 * item.distance as u16
                + ((1.0 - health.powi(2)) * 100.0) as u16
        }
    })?;

    Some(target_sector.sector)
}

#[must_use]
pub fn entity_distance_to_ship_squared<T>(
    all_transforms: &Query<&SimulationTransform>,
    ship_pos: Vec2,
    a: T,
) -> u32
where
    T: Into<Entity>,
{
    all_transforms
        .get(a.into())
        .unwrap()
        .translation
        .distance_squared(ship_pos) as u32
}

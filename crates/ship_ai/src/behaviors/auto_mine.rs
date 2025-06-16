use crate::utility::task_filters::ShipIsIdleFilter;
use crate::utility::trade_plan::TradePlan;
use bevy::prelude::{Entity, EventWriter, Mut, Query, Res, Vec2};
use common::components::ship_behavior::ShipBehavior;
use common::components::{BuyOrders, InSector, Inventory, Sector, SectorWithAsteroids};
use common::events::task_events::{InsertTaskIntoQueueCommand, TaskInsertionMode};
use common::game_data::{ItemId, ItemManifest};
use common::simulation_time::SimulationTime;
use common::simulation_transform::SimulationTransform;
use common::types::auto_mine_state::AutoMineState;
use common::types::entity_wrappers::SectorEntity;
use common::types::exchange_ware_data::ExchangeWareData;
use common::types::ship_behaviors::AutoMineBehavior;
use common::types::ship_tasks::{ExchangeWares, MineAsteroid, MoveToSector};

#[allow(clippy::too_many_arguments)]
pub fn handle_idle_ships(
    simulation_time: Res<SimulationTime>,
    mut ships: Query<(Entity, &mut ShipBehavior<AutoMineBehavior>, &InSector), ShipIsIdleFilter>,
    buy_orders: Query<(Entity, &mut BuyOrders, &InSector)>,
    mut inventories: Query<&mut Inventory>,
    all_sectors_with_asteroids: Query<&SectorWithAsteroids>,
    all_sectors: Query<&Sector>,
    all_transforms: Query<&SimulationTransform>,
    item_manifest: Res<ItemManifest>,
    mut mine_asteroid_event_writer: EventWriter<InsertTaskIntoQueueCommand<MineAsteroid>>,
    mut exchange_wares_event_writer: EventWriter<InsertTaskIntoQueueCommand<ExchangeWares>>,
    mut move_to_sector_event_writer: EventWriter<InsertTaskIntoQueueCommand<MoveToSector>>,
) {
    let now = simulation_time.now();

    // Avoids selecting an asteroid which is close to leaving the sector
    let max_asteroid_age = now.add_milliseconds(15000);

    // TODO: Benchmark this .filter vs a priority queue
    ships
        .iter_mut()
        .filter(|(_, behavior, _)| now.has_passed(behavior.next_idle_update))
        .for_each(|(ship_entity, mut behavior, in_sector)| {
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
                            .min_by_key(|&asteroid| {
                                entity_distance_to_ship_squared(&all_transforms, ship_pos, asteroid)
                            })
                        {
                            mine_asteroid_event_writer.write(InsertTaskIntoQueueCommand {
                                entity: ship_entity,
                                insertion_mode: TaskInsertionMode::Append,
                                task_data: MineAsteroid::new(closest_asteroid.entity),
                            });
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

                    move_to_sector_event_writer.write(InsertTaskIntoQueueCommand {
                        entity: ship_entity,
                        insertion_mode: TaskInsertionMode::Append,
                        task_data: MoveToSector {
                            sector: target_sector,
                        },
                    });
                }
                AutoMineState::Trading => {
                    if try_sell_everything_in_inventory(
                        &buy_orders,
                        &mut exchange_wares_event_writer,
                        ship_entity,
                        in_sector,
                        &ship_inventory,
                    )
                    .is_err()
                    {
                        behavior.next_idle_update = now.add_milliseconds(2000);
                    }
                }
            }
        });
}

/// Tries to create tasks to sell stuff from this entities' inventory.
/// Right now this will only sell one thing at a time, so the name might be a tad misleading. :)
///
/// # Returns
/// Ok if new tasks where created, Err otherwise.
pub fn try_sell_everything_in_inventory(
    buy_orders: &Query<(Entity, &mut BuyOrders, &InSector)>,
    exchange_wares_event_writer: &mut EventWriter<InsertTaskIntoQueueCommand<ExchangeWares>>,
    ship_entity: Entity,
    in_sector: &InSector,
    ship_inventory: &Mut<Inventory>,
) -> Result<(), ()> {
    let Some(plan) =
        TradePlan::sell_anything_from_inventory(ship_entity, in_sector, ship_inventory, buy_orders)
    else {
        return Err(());
    };

    exchange_wares_event_writer.write(InsertTaskIntoQueueCommand {
        entity: ship_entity,
        insertion_mode: TaskInsertionMode::Append,
        task_data: ExchangeWares::new(
            plan.buyer,
            ExchangeWareData::Sell(plan.item_id, plan.amount),
        ),
    });

    Ok(())
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

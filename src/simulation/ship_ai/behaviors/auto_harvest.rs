use crate::components::{
    BuyOrders, GasGiant, InSector, InventoryComponent, SectorComponent, SectorPlanetsComponent,
};
use crate::game_data::{ItemId, ItemManifest};
use crate::pathfinding;
use crate::simulation::prelude::{SimulationTime, SimulationTimestamp};
use crate::simulation::ship_ai::behaviors::auto_mine;
use crate::simulation::ship_ai::ship_is_idle_filter::ShipIsIdleFilter;
use crate::simulation::ship_ai::task_started_event::AllTaskStartedEventWriters;
use crate::simulation::ship_ai::{TaskInsideQueue, TaskQueue};
use crate::simulation::transform::simulation_transform::SimulationTransform;
use crate::trade_plan::TradePlan;
use crate::utils::{SectorEntity, TradeIntent, TypedEntity};
use bevy::prelude::{Commands, Component, Entity, Query, Res};

#[derive(Component)]
pub struct AutoHarvestBehavior {
    // TODO: Maybe(?) could just be AutoMineBehavior<T> with T: MineAsteroid | HarvestGas
    pub next_idle_update: SimulationTimestamp,
    pub state: auto_mine::AutoMineState,
    pub harvested_gas: ItemId,
}

#[allow(clippy::too_many_arguments)]
pub fn handle_idle_ships(
    mut commands: Commands,
    simulation_time: Res<SimulationTime>,
    mut ships: Query<
        (Entity, &mut TaskQueue, &mut AutoHarvestBehavior, &InSector),
        ShipIsIdleFilter,
    >,
    buy_orders: Query<(Entity, &mut BuyOrders, &InSector)>,
    mut inventories: Query<&mut InventoryComponent>,
    all_sectors_with_gas_giants: Query<&SectorPlanetsComponent>,
    all_sectors: Query<&SectorComponent>,
    all_gas_giants: Query<&GasGiant>,
    all_transforms: Query<&SimulationTransform>,
    item_manifest: Res<ItemManifest>,
    mut all_task_started_event_writers: AllTaskStartedEventWriters,
) {
    let now = simulation_time.now();

    ships
        .iter_mut()
        .filter(|(_, _, behavior, _)| now.has_passed(behavior.next_idle_update))
        .for_each(|(ship_entity, mut queue, mut behavior, in_sector)| {
            let ship_inventory = inventories.get_mut(ship_entity).unwrap();
            let used_space = ship_inventory.total_used_space();
            let remaining_space =
                ship_inventory.remaining_space_for(&behavior.harvested_gas, &item_manifest);

            behavior
                .state
                .flip_task_depending_on_inventory(used_space, remaining_space);

            match behavior.state {
                auto_mine::AutoMineState::Mining => {
                    if let Ok(sector_planets) =
                        all_sectors_with_gas_giants.get(in_sector.sector.into())
                    {
                        let ship_pos = all_transforms.get(ship_entity).unwrap().translation;

                        if let Some(closest_planet) = sector_planets
                            .planets
                            .iter()
                            .filter(|&x| all_gas_giants.get(x.into()).is_ok())
                            .min_by_key(|&planet| {
                                auto_mine::entity_distance_to_ship_squared(
                                    &all_transforms,
                                    ship_pos,
                                    planet,
                                )
                            })
                        {
                            queue.push_back(TaskInsideQueue::MoveToEntity {
                                target: TypedEntity::Planet(*closest_planet),
                                stop_at_target: true,
                                distance_to_target: 0.0,
                            });
                            queue.push_back(TaskInsideQueue::RequestAccess {
                                target: TypedEntity::Planet(*closest_planet),
                            });
                            queue.push_back(TaskInsideQueue::HarvestGas {
                                target: *closest_planet,
                                gas: behavior.harvested_gas,
                            });

                            queue.apply(
                                &mut commands,
                                now,
                                ship_entity,
                                &mut all_task_started_event_writers,
                            );
                            return;
                        }
                    }

                    // No planets available in current sector, go somewhere else!
                    let target_sector = match find_nearby_sector_with_gas_giants(
                        &all_gas_giants,
                        &all_sectors_with_gas_giants,
                        &all_sectors,
                        in_sector,
                        &behavior.harvested_gas,
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
                    queue.apply(
                        &mut commands,
                        now,
                        ship_entity,
                        &mut all_task_started_event_writers,
                    );
                }
                auto_mine::AutoMineState::Trading => {
                    // TODO: This is quite literally 100% the same logic as auto_mine
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
                    queue.apply(
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
fn find_nearby_sector_with_gas_giants(
    all_gas_giants: &Query<&GasGiant>,
    all_sectors_with_planets: &Query<&SectorPlanetsComponent>,
    all_sectors: &Query<&SectorComponent>,
    in_sector: &InSector,
    gas: &ItemId,
) -> Option<SectorEntity> {
    let nearby_sectors_with_asteroids =
        pathfinding::surrounding_sector_search::surrounding_sector_search(
            all_sectors,
            in_sector.sector,
            1,
            u8::MAX, // TODO: Should be limited
            all_sectors_with_planets,
            |x| {
                x.planets.iter().any(|x| {
                    if let Ok(gas_giant) = all_gas_giants.get(x.into()) {
                        gas_giant.resources.contains(gas)
                    } else {
                        false
                    }
                })
            },
        );

    let target_sector = nearby_sectors_with_asteroids.iter().min()?;
    Some(target_sector.sector)
}

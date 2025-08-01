use crate::behaviors::auto_mine;
use crate::utility::task_filters::ShipIsIdleFilter;
use bevy::prelude::{Entity, EventWriter, Query, Res};
use common::components::celestials::GasGiant;
use common::components::ship_behavior::ShipBehavior;
use common::components::{BuyOrders, InSector, Inventory, Sector, SectorWithCelestials};
use common::events::task_events::{InsertTaskIntoQueueCommand, TaskInsertionMode};
use common::game_data::{ItemId, ItemManifest};
use common::simulation_time::SimulationTime;
use common::simulation_transform::SimulationTransform;
use common::types::auto_mine_state;
use common::types::entity_wrappers::SectorEntity;
use common::types::ship_behaviors::AutoHarvestBehavior;
use common::types::ship_tasks::{ExchangeWares, HarvestGas, MoveToSector};

#[allow(clippy::too_many_arguments)]
pub fn handle_idle_ships(
    simulation_time: Res<SimulationTime>,
    mut ships: Query<(Entity, &mut ShipBehavior<AutoHarvestBehavior>, &InSector), ShipIsIdleFilter>,
    buy_orders: Query<(Entity, &mut BuyOrders, &InSector)>,
    mut inventories: Query<&mut Inventory>,
    all_sectors_with_gas_giants: Query<&SectorWithCelestials>,
    all_sectors: Query<&Sector>,
    all_gas_giants: Query<&GasGiant>,
    all_transforms: Query<&SimulationTransform>,
    item_manifest: Res<ItemManifest>,
    mut harvest_gas_event_writer: EventWriter<InsertTaskIntoQueueCommand<HarvestGas>>,
    mut exchange_wares_event_writer: EventWriter<InsertTaskIntoQueueCommand<ExchangeWares>>,
    mut move_to_sector_event_writer: EventWriter<InsertTaskIntoQueueCommand<MoveToSector>>,
) {
    let now = simulation_time.now();
    ships
        .iter_mut()
        .filter(|(_, behavior, _)| now.has_passed(behavior.next_idle_update))
        .for_each(|(ship_entity, mut behavior, in_sector)| {
            let ship_inventory = inventories.get_mut(ship_entity).unwrap();
            let used_space = ship_inventory.total_used_space();
            let remaining_space =
                ship_inventory.remaining_space_for(&behavior.harvested_gas, &item_manifest);

            behavior
                .state
                .flip_task_depending_on_inventory(used_space, remaining_space);

            match behavior.state {
                auto_mine_state::AutoMineState::Mining => {
                    if let Ok(sector_planets) =
                        all_sectors_with_gas_giants.get(in_sector.sector.into())
                    {
                        let ship_pos = all_transforms.get(ship_entity).unwrap().translation;

                        if let Some(closest_planet) = sector_planets
                            .gas_giants
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
                            harvest_gas_event_writer.write(InsertTaskIntoQueueCommand {
                                entity: ship_entity,
                                insertion_mode: TaskInsertionMode::Append,
                                task_data: HarvestGas::new(*closest_planet, behavior.harvested_gas),
                            });
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

                    move_to_sector_event_writer.write(InsertTaskIntoQueueCommand {
                        entity: ship_entity,
                        insertion_mode: TaskInsertionMode::Append,
                        task_data: MoveToSector {
                            sector: target_sector,
                        },
                    });
                }
                auto_mine_state::AutoMineState::Trading => {
                    if auto_mine::try_sell_everything_in_inventory(
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

#[must_use]
fn find_nearby_sector_with_gas_giants(
    all_gas_giants: &Query<&GasGiant>,
    all_sectors_with_celestials: &Query<&SectorWithCelestials>,
    all_sectors: &Query<&Sector>,
    in_sector: &InSector,
    gas: &ItemId,
) -> Option<SectorEntity> {
    let nearby_sectors_with_asteroids =
        pathfinding::surrounding_sector_search::surrounding_sector_search(
            all_sectors,
            in_sector.sector,
            1,
            u8::MAX, // TODO: Should be limited
            all_sectors_with_celestials,
            |sector_with_celestials| {
                sector_with_celestials.gas_giants.iter().any(|x| {
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

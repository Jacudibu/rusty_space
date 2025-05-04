use crate::components::{
    BuyOrders, ConstructionSite, ConstructionSiteStatus, InSector, Inventory, Sector, Station,
};
use crate::game_data::{
    Constructable, ConstructableModuleId, ItemId, ItemManifest, ProductionModuleManifest,
    ShipyardModuleManifest,
};
use crate::simulation::prelude::{
    Construct, ProductionFacility, ProductionModule, Shipyard, ShipyardModule,
};
use crate::simulation::ship_ai::TaskCompletedEvent;
use crate::utils::ConstructionSiteEntity;
use bevy::platform::collections::HashSet;
use bevy::prelude::{
    App, Commands, Entity, Event, EventReader, EventWriter, FixedUpdate, IntoScheduleConfigs,
    Plugin, Query, Res, Time, error, in_state,
};
use common::states::SimulationState;
use std::ops::Not;

pub struct ConstructionSiteUpdaterPlugin;

impl Plugin for ConstructionSiteUpdaterPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ConstructionFinishedEvent>();
        app.add_systems(
            FixedUpdate,
            (construction_site_updater, construction_site_finisher)
                .chain()
                .run_if(in_state(SimulationState::Running)),
        );
    }
}

#[derive(Event)]
pub struct ConstructionFinishedEvent {
    pub entity: ConstructionSiteEntity,
}

fn construction_site_updater(
    time: Res<Time>,
    mut all_construction_sites: Query<(
        Entity,
        &mut ConstructionSite,
        &mut Inventory,
        &mut BuyOrders,
    )>,
    production_modules: Res<ProductionModuleManifest>,
    shipyard_modules: Res<ShipyardModuleManifest>,
    item_manifest: Res<ItemManifest>,
    mut event_writer: EventWriter<ConstructionFinishedEvent>,
) {
    let delta = time.delta_secs();

    all_construction_sites.iter_mut().for_each(
        |(entity, mut site, mut inventory, mut buy_orders)| {
            let module = site.build_order.first().unwrap();
            // TODO: construction site data should be available through a sub-struct on each constructable
            let constructable_data = match module {
                ConstructableModuleId::ProductionModule(id) => production_modules
                    .get_by_ref(id)
                    .unwrap()
                    .get_constructable_data(),
                ConstructableModuleId::ShipyardModule(id) => shipyard_modules
                    .get_by_ref(id)
                    .unwrap()
                    .get_constructable_data(),
            };

            if site.progress_until_next_step <= site.current_build_progress {
                let mut missing_materials = HashSet::new();
                let ingredients_for_step =
                    &constructable_data.required_materials_per_step[site.next_construction_step];
                for ingredient in ingredients_for_step {
                    let Some(inventory_element) = inventory.get(&ingredient.item_id) else {
                        missing_materials.insert(ingredient.item_id);
                        continue;
                    };

                    if inventory_element.available_right_now() < ingredient.amount {
                        missing_materials.insert(ingredient.item_id);
                        continue;
                    }
                }

                if missing_materials.is_empty().not() {
                    let mut missing_items = missing_materials.into_iter().collect::<Vec<ItemId>>();
                    missing_items.sort();
                    site.status = ConstructionSiteStatus::MissingMaterials(missing_items);
                    return;
                }

                // TODO: Maybe move the rest below the point where we set the state to missing builders so resources are only consumed when building actually happens
                for ingredient in ingredients_for_step {
                    buy_orders
                        .orders
                        .get_mut(&ingredient.item_id)
                        .unwrap()
                        .amount -= ingredient.amount;
                    inventory.remove_item(ingredient.item_id, ingredient.amount, &item_manifest);
                }

                site.next_construction_step += 1;
                if site.next_construction_step
                    == constructable_data.required_materials_per_step.len()
                {
                    // Final step, all necessary materials have been consumed, no need to check this ever again. Also allows overflowing build power into next module.
                    site.progress_until_next_step = f32::MAX;
                } else {
                    site.progress_until_next_step += constructable_data.progress_per_step;
                }
            }

            if site.construction_ships.is_empty() {
                site.status = ConstructionSiteStatus::MissingBuilders;
            }

            let now_progress =
                site.current_build_progress + site.total_build_power_of_ships as f32 * delta;
            site.current_build_progress = site.progress_until_next_step.min(now_progress);
            site.status = ConstructionSiteStatus::Ok;

            if site.current_build_progress as u32 >= constructable_data.required_build_power {
                site.current_build_progress -= constructable_data.required_build_power as f32;
                event_writer.write(ConstructionFinishedEvent {
                    entity: entity.into(),
                });
            }
        },
    );
}

fn construction_site_finisher(
    mut commands: Commands,
    mut events: EventReader<ConstructionFinishedEvent>,
    mut all_construction_sites: Query<(&mut ConstructionSite, &InSector)>,
    mut all_stations: Query<(
        &mut Station,
        Option<&mut ProductionFacility>,
        Option<&mut Shipyard>,
    )>,
    mut task_finished_event_writer: EventWriter<TaskCompletedEvent<Construct>>,
    mut all_sectors: Query<&mut Sector>,
) {
    for event in events.read() {
        let (mut construction_site, in_sector) =
            all_construction_sites.get_mut(event.entity.into()).unwrap();
        let (mut station, production, shipyards) = all_stations
            .get_mut(construction_site.station.into())
            .unwrap();

        let Some(finished_thing) = construction_site.build_order.pop() else {
            error!(
                "Construction Site {:?} didn't contain any construction modules!",
                event.entity
            );
            continue;
        };
        match finished_thing {
            ConstructableModuleId::ProductionModule(id) => {
                if let Some(mut production) = production {
                    match production.modules.get_mut(&id) {
                        None => {
                            production.modules.insert(id, ProductionModule::default());
                        }
                        Some(module) => {
                            module.amount += 1;
                            // TODO: Send some "New Production module ready" event so stations start using it immediately
                        }
                    }
                } else {
                    let component = ProductionFacility {
                        modules: [(id, ProductionModule::default())].into(),
                    };

                    commands
                        .entity(construction_site.station.into())
                        .insert(component);
                }
            }
            ConstructableModuleId::ShipyardModule(id) => {
                if let Some(mut shipyards) = shipyards {
                    match shipyards.modules.get_mut(&id) {
                        None => {
                            shipyards.modules.insert(id, ShipyardModule::default());
                        }
                        Some(module) => {
                            module.amount += 1;
                            // TODO: Send some "New Shipyard module ready" event so shipyards can queue an additional ship
                        }
                    }
                } else {
                    let component = Shipyard {
                        modules: [(id, ShipyardModule::default())].into(),
                        queue: Default::default(),
                    };

                    commands
                        .entity(construction_site.station.into())
                        .insert(component);
                }
            }
        }

        if construction_site.build_order.is_empty() {
            // TODO: This should probably also be handled as an event?
            station.construction_site = None;
            task_finished_event_writer.write_batch(
                construction_site
                    .construction_ships
                    .iter()
                    .map(|x| TaskCompletedEvent::new(*x)),
            );
            all_sectors
                .get_mut(in_sector.into())
                .unwrap()
                .remove_construction_site(event.entity);
            commands.entity(event.entity.into()).despawn();
        }
    }
}

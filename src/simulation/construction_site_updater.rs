use crate::components::{
    ConstructionSiteComponent, ConstructionSiteStatus, InSector, SectorComponent, StationComponent,
};
use crate::game_data::{ConstructableModuleId, ProductionModuleManifest, ShipyardModuleManifest};
use crate::simulation::prelude::{
    ConstructTaskComponent, ProductionComponent, ProductionModule, ShipyardComponent,
    ShipyardModule,
};
use crate::utils::ConstructionSiteEntity;
use bevy::app::{App, FixedUpdate};
use bevy::prelude::{
    Commands, Entity, Event, EventReader, EventWriter, Fixed, IntoSystemConfigs, Plugin, Query,
    Res, Time, error,
};
use leafwing_manifest::manifest::Manifest;

pub struct ConstructionSiteUpdaterPlugin;

impl Plugin for ConstructionSiteUpdaterPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ConstructionFinishedEvent>();
        app.add_systems(
            FixedUpdate,
            (construction_site_updater, construction_site_finisher).chain(),
        );
    }
}

#[derive(Event)]
pub struct ConstructionFinishedEvent {
    pub entity: ConstructionSiteEntity,
}

fn construction_site_updater(
    time: Res<Time<Fixed>>,
    mut all_construction_sites: Query<(Entity, &mut ConstructionSiteComponent)>,
    production_modules: Res<ProductionModuleManifest>,
    shipyard_modules: Res<ShipyardModuleManifest>,
    mut event_writer: EventWriter<ConstructionFinishedEvent>,
) {
    let delta = time.delta_secs();

    all_construction_sites
        .iter_mut()
        .for_each(|(entity, mut site)| {
            // TODO: Check for missing materials

            // TODO: Consume materials if there are enough to progress.

            // TODO: Persist how far we can progress given the consumed materials

            if site.construction_ships.is_empty() {
                site.status = ConstructionSiteStatus::MissingBuilders;
            }

            site.current_build_progress += site.total_build_power as f32 * delta;
            site.status = ConstructionSiteStatus::Ok;

            let module = site.build_order.first().unwrap();
            let required_build_power = match module {
                ConstructableModuleId::ProductionModule(id) => {
                    production_modules
                        .get_by_ref(id)
                        .unwrap()
                        .required_build_power
                }
                ConstructableModuleId::ShipyardModule(id) => {
                    shipyard_modules
                        .get_by_ref(id)
                        .unwrap()
                        .required_build_power
                }
            };

            if site.current_build_progress as u32 > required_build_power {
                event_writer.send(ConstructionFinishedEvent {
                    entity: entity.into(),
                });
            }
        });
}

fn construction_site_finisher(
    mut commands: Commands,
    mut events: EventReader<ConstructionFinishedEvent>,
    mut all_construction_sites: Query<(Entity, &mut ConstructionSiteComponent, &InSector)>,
    mut all_stations: Query<(
        &mut StationComponent,
        Option<&mut ProductionComponent>,
        Option<&mut ShipyardComponent>,
    )>,
    mut all_sectors: Query<&mut SectorComponent>,
    production_manifest: Res<ProductionModuleManifest>,
) {
    for event in events.read() {
        let (entity, mut construction_site, in_sector) =
            all_construction_sites.get_mut(event.entity.into()).unwrap();
        let (mut station, production, shipyards) = all_stations
            .get_mut(construction_site.station.into())
            .unwrap();

        let Some(finished_thing) = construction_site.build_order.pop() else {
            error!("Construction Site {entity:?} didn't contain any construction modules!");
            continue;
        };
        match finished_thing {
            ConstructableModuleId::ProductionModule(id) => {
                let Some(mut production) = production else {
                    todo!();
                };

                match production.modules.get_mut(&id) {
                    None => {
                        let recipes = &production_manifest.get(id).unwrap().available_recipes;

                        production.modules.insert(
                            id,
                            ProductionModule {
                                recipe: *recipes.first().unwrap(), // TODO: Guess this needs to be an option after all! Wouldn't want to start a random recipe... Or maybe this could already be part of the construction order?
                                amount: 1,
                                current_run_finished_at: None,
                            },
                        );
                    }
                    Some(module) => {
                        module.amount += 1; // TODO: This shouldn't increase the amount of running recipes, so things need to be split
                    }
                }
            }
            ConstructableModuleId::ShipyardModule(id) => {
                let Some(mut shipyards) = shipyards else {
                    todo!();
                };
                match shipyards.modules.get_mut(&id) {
                    None => {
                        // TODO
                        shipyards.modules.insert(
                            id,
                            ShipyardModule {
                                amount: 1,
                                active: Vec::new(),
                            },
                        );
                    }
                    Some(module) => {
                        module.amount += 1;
                        // TODO: Send some "New Shipyard module ready" event so shipyards can queue an additional ship
                    }
                }
            }
        }

        if construction_site.build_order.is_empty() {
            // TODO: This should probably also be handled as an event?
            station.construction_site = None;
            for x in &construction_site.construction_ships {
                commands.entity(x.into()).remove::<ConstructTaskComponent>();
            }

            all_sectors
                .get_mut(in_sector.into())
                .unwrap()
                .remove_construction_site(entity.into());
            commands.entity(entity).despawn();
        }
    }
}

use crate::components::{
    ConstructionSiteComponent, ConstructionSiteStatus, InSector, SectorComponent, StationComponent,
};
use crate::game_data::{ConstructableModuleId, ProductionModuleManifest, ShipyardModuleManifest};
use crate::simulation::prelude::{
    ConstructTaskComponent, ProductionComponent, ProductionModule, ShipyardComponent,
    ShipyardModule,
};
use crate::simulation::ship_ai::TaskFinishedEvent;
use crate::states::SimulationState;
use crate::utils::ConstructionSiteEntity;
use bevy::prelude::{
    App, Commands, Entity, Event, EventReader, EventWriter, Fixed, FixedUpdate, IntoSystemConfigs,
    Plugin, Query, Res, Time, error, in_state,
};

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
                site.current_build_progress -= required_build_power as f32;
                event_writer.send(ConstructionFinishedEvent {
                    entity: entity.into(),
                });
            }
        });
}

fn construction_site_finisher(
    mut commands: Commands,
    mut events: EventReader<ConstructionFinishedEvent>,
    mut all_construction_sites: Query<(&mut ConstructionSiteComponent, &InSector)>,
    mut all_stations: Query<(
        &mut StationComponent,
        Option<&mut ProductionComponent>,
        Option<&mut ShipyardComponent>,
    )>,
    mut task_finished_event_writer: EventWriter<TaskFinishedEvent<ConstructTaskComponent>>,
    mut all_sectors: Query<&mut SectorComponent>,
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
                    let component = ProductionComponent {
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
                    let component = ShipyardComponent {
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
            task_finished_event_writer.send_batch(
                construction_site
                    .construction_ships
                    .iter()
                    .map(|x| TaskFinishedEvent::new(x.into())),
            );
            all_sectors
                .get_mut(in_sector.into())
                .unwrap()
                .remove_construction_site(event.entity);
            commands.entity(event.entity.into()).despawn();
        }
    }
}

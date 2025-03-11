use crate::components::{ConstructionSiteComponent, ConstructionSiteStatus};
use crate::game_data::{ConstructableModuleId, ProductionModuleManifest, ShipyardModuleManifest};
use crate::simulation::prelude::TaskFinishedEvent;
use crate::utils::ConstructionSiteEntity;
use bevy::app::{App, FixedUpdate};
use bevy::prelude::{Entity, Event, Fixed, Plugin, Query, Res, Time};
use std::sync::{Arc, Mutex};

pub struct ConstructionSiteUpdaterPlugin;

impl Plugin for ConstructionSiteUpdaterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, construction_site_updater);
    }
}

#[derive(Event)]
struct ConstructionFinishedEvent {
    entity: ConstructionSiteEntity,
}

fn construction_site_updater(
    time: Res<Time<Fixed>>,
    mut all_construction_sites: Query<(Entity, &mut ConstructionSiteComponent)>,
    production_modules: Res<ProductionModuleManifest>,
    shipyard_modules: Res<ShipyardModuleManifest>,
) {
    let completions = Arc::new(Mutex::new(Vec::<ConstructionFinishedEvent>::new()));
    let delta = time.delta_secs();

    all_construction_sites
        .par_iter_mut()
        .for_each(|(entity, mut site)| {
            // TODO: Check for missing materials

            // TODO: Consume materials if there are enough to progress.

            // TODO: Persist how far we can progress given the consumed materials

            if site.construction_ship_count == 0 {
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
                completions.lock().unwrap().push(ConstructionFinishedEvent {
                    entity: entity.into(),
                })
            }
        });

    // TODO: write events
}

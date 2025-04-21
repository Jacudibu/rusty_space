use crate::components::{ConstructionSiteComponent, Ship};
use crate::session_data::ShipConfigurationManifest;
use crate::simulation::prelude::TaskComponent;
use crate::simulation::ship_ai::task_events::TaskStartedEvent;
use crate::utils::{ConstructionSiteEntity, ShipEntity};
use bevy::prelude::{Component, EventReader, Query, Res, error};

/// Ships with this [TaskComponent] are actively working on a construction site.
#[derive(Component)]
pub struct ConstructTaskComponent {
    pub target: ConstructionSiteEntity,
}
impl TaskComponent for ConstructTaskComponent {
    fn can_be_aborted() -> bool {
        true
    }
}

impl ConstructTaskComponent {
    pub fn on_task_started(
        construction_tasks: Query<(&Self, &Ship)>,
        mut construction_sites: Query<&mut ConstructionSiteComponent>,
        mut event_reader: EventReader<TaskStartedEvent<Self>>,
        ship_configurations: Res<ShipConfigurationManifest>,
    ) {
        for event in event_reader.read() {
            let (task, ship) = construction_tasks.get(event.entity.into()).unwrap();
            let mut construction_site = construction_sites.get_mut(task.target.into()).unwrap();
            let ship_config = ship_configurations.get_by_id(&ship.config_id()).unwrap();
            let Some(build_power) = ship_config.computed_stats.build_power else {
                error!(
                    "Attempted to start construction task on ship without build power: {:?}",
                    event.entity
                );
                continue;
            };

            add_builder(&mut construction_site, build_power, event.entity);
        }
    }

    pub fn run_tasks() {
        // Individual ships don't do anything whilst constructing, that's handled inside construction_site_updater
    }

    pub fn cancel_task() {
        // remove build_power from construction site
    }
}

fn add_builder(site: &mut ConstructionSiteComponent, build_power: u32, entity: ShipEntity) {
    site.total_build_power_of_ships += build_power;
    site.construction_ships.insert(entity);
}

fn remove_builder(site: &mut ConstructionSiteComponent, build_power: u32, entity: &ShipEntity) {
    site.total_build_power_of_ships -= build_power;
    site.construction_ships.remove(entity);
}

use crate::ship_ai::TaskComponent;
use crate::ship_ai::ship_task::ShipTask;
use bevy::prelude::{EventReader, Query, Res, error};
use common::components::{ConstructionSite, Ship};
use common::events::task_events::{TaskCanceledWhileActiveEvent, TaskStartedEvent};
use common::session_data::ShipConfigurationManifest;
use common::types::entity_wrappers::ShipEntity;
use common::types::ship_tasks::Construct;

impl TaskComponent for ShipTask<Construct> {
    fn can_be_cancelled_while_active() -> bool {
        true
    }
}

impl ShipTask<Construct> {
    pub fn on_task_started(
        construction_tasks: Query<(&Self, &Ship)>,
        mut construction_sites: Query<&mut ConstructionSite>,
        mut event_reader: EventReader<TaskStartedEvent<Construct>>,
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

            register_ship(&mut construction_site, event.entity, build_power);
        }
    }

    pub(crate) fn run_tasks() {
        // Individual ships don't do anything whilst constructing, that's handled inside construction_site_updater
    }

    pub(crate) fn cancel_task_inside_queue() {
        // Nothing needs to be done.
    }

    pub(crate) fn abort_running_task(
        mut cancelled_tasks: EventReader<TaskCanceledWhileActiveEvent<Construct>>,
        mut construction_sites: Query<&mut ConstructionSite>,
    ) {
        for event in cancelled_tasks.read() {
            let Ok(mut site) = construction_sites.get_mut(event.task_data.target.into()) else {
                continue;
            };

            deregister_ship(&mut site, event.entity.into());
        }
    }
}

/// Registers a ship as an active worker for the provided [ConstructionSite].
pub fn register_ship(site: &mut ConstructionSite, entity: ShipEntity, build_power: u32) {
    site.total_build_power_of_ships += build_power;
    if let Some(old_value) = site.construction_ships.insert(entity, build_power) {
        site.total_build_power_of_ships -= old_value;
    }
}

/// Removes a ship registration from the provided [ConstructionSite]
pub fn deregister_ship(site: &mut ConstructionSite, entity: ShipEntity) {
    if let Some(build_power) = site.construction_ships.remove(&entity) {
        site.total_build_power_of_ships -= build_power;
    }
}

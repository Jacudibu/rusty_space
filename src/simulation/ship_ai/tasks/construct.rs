use crate::components::{ConstructionSiteComponent, Ship};
use crate::session_data::ShipConfigurationManifest;
use crate::simulation::prelude::{SimulationTime, TaskFinishedEvent, TaskQueue};
use crate::simulation::ship_ai::task_started_event::{
    AllTaskStartedEventWriters, TaskStartedEvent,
};
use crate::simulation::ship_ai::tasks;
use crate::utils::{ConstructionSiteEntity, ShipEntity};
use bevy::prelude::{Commands, Component, EventReader, Query, Res, With, error};

#[derive(Component)]
pub struct ConstructTaskComponent {
    pub target: ConstructionSiteEntity,
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

    pub fn complete_tasks(
        mut commands: Commands,
        mut event_reader: EventReader<TaskFinishedEvent<Self>>,
        mut all_ships_with_task: Query<&mut TaskQueue, With<Self>>,
        simulation_time: Res<SimulationTime>,
        mut task_started_event_writers: AllTaskStartedEventWriters,
    ) {
        let now = simulation_time.now();

        for event in event_reader.read() {
            if let Ok(mut queue) = all_ships_with_task.get_mut(event.entity) {
                tasks::remove_task_and_add_next_in_queue::<Self>(
                    &mut commands,
                    event.entity,
                    &mut queue,
                    now,
                    &mut task_started_event_writers,
                );
            } else {
                error!(
                    "Unable to find entity for task completion: {}",
                    event.entity
                );
            }
        }
    }
}

fn add_builder(site: &mut ConstructionSiteComponent, build_power: u32, entity: ShipEntity) {
    site.total_build_power += build_power;
    site.construction_ships.insert(entity);
}

fn remove_builder(site: &mut ConstructionSiteComponent, build_power: u32, entity: &ShipEntity) {
    site.total_build_power -= build_power;
    site.construction_ships.remove(entity);
}

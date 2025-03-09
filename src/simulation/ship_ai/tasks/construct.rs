use crate::components::{ConstructionSiteComponent, Ship};
use crate::simulation::ship_ai::task_started_event::TaskStartedEvent;
use crate::utils::ConstructionSiteEntity;
use bevy::prelude::{Component, EventReader, Query};

#[derive(Component)]
pub struct Construct {
    pub target: ConstructionSiteEntity,
}

impl Construct {
    pub fn on_task_started(
        construction_tasks: Query<(&Self, &Ship)>,
        construction_sites: Query<&mut ConstructionSiteComponent>,
        mut event_reader: EventReader<TaskStartedEvent<Self>>,
    ) {
        // for event in event_reader.read() {
        //     let (task, ship) = construction_tasks.get(event.entity).unwrap();
        //     let construction_site = construction_sites.get(event.entity).unwrap();
        //     construction_site.total_construction_power += ship..add(ship)
        // }
        // Add buildpower to construction site
    }

    pub fn run_tasks() {}

    pub fn cancel_task() {
        // remove buildpoewr from construction site
    }

    pub fn complete_tasks() {
        // since the build site disappears when construction is finished, being unable to find the related entity is our completion condition
    }
}

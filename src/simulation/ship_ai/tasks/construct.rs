use crate::utils::ConstructionSiteEntity;
use bevy::prelude::Component;

#[derive(Component)]
pub struct Construct {
    pub target: ConstructionSiteEntity,
}

fn start_task() {
    // Add buildpower to construction site
}

fn cancel_task() {
    // remove buildpoewr from construction site
}

fn complete_task() {
    // since the build site disappears when construction is finished, being unable to find the related entity is our completion condition
}

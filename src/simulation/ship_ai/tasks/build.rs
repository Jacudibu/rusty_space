use crate::utils::BuildSiteEntity;
use bevy::prelude::Component;

#[derive(Component)]
pub struct Build {
    pub target: BuildSiteEntity,
}

fn start_task() {
    // Add buildpower to construction site
}

fn cancel_task() {
    // remove buildpoewr from construction site
}

fn complete_task() {
    // since the build site disappears when building is finished, being unable to find the related entity is our completion condition
}

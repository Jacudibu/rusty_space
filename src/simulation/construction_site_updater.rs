use crate::components::{ConstructionSiteComponent, ConstructionSiteStatus};
use bevy::app::{App, FixedUpdate};
use bevy::prelude::{Fixed, Plugin, Query, Res, Time};

pub struct ConstructionSiteUpdaterPlugin;

impl Plugin for ConstructionSiteUpdaterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, construction_site_updater);
    }
}

fn construction_site_updater(
    time: Res<Time<Fixed>>,
    mut all_construction_sites: Query<&mut ConstructionSiteComponent>,
) {
    let delta = time.delta_secs();

    all_construction_sites.par_iter_mut().for_each(|mut site| {
        // TODO: Check for missing materials

        // TODO: Consume materials if there are enough to progress.

        // TODO: Persist how far we can progress given the consumed materials

        if site.construction_ship_count == 0 {
            site.status = ConstructionSiteStatus::MissingBuilders;
        }

        site.current_build_progress += site.total_construction_power as f32 * delta;
        site.status = ConstructionSiteStatus::Ok;

        // TODO: check if construction is finished
    });
}

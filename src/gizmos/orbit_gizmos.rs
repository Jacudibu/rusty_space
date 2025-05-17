use bevy::app::{App, Plugin, Startup, Update};
use bevy::prelude::{
    AppGizmoBuilder, GizmoConfigGroup, GizmoConfigStore, Gizmos, Query, Reflect, ResMut, With,
};

use common::components::{ConstantOrbit, InSector, Sector};
use common::constants;
use entity_selection::components::IsEntitySelected;

pub(crate) struct OrbitGizmoPlugin;
impl Plugin for OrbitGizmoPlugin {
    fn build(&self, app: &mut App) {
        app.init_gizmo_group::<OrbitLineGizmos>()
            .add_systems(Startup, configure)
            .add_systems(Update, draw_orbit_circles);
    }
}

#[derive(Default, Reflect, GizmoConfigGroup)]
struct OrbitLineGizmos;

fn configure(mut config_store: ResMut<GizmoConfigStore>) {
    let (config, _) = config_store.config_mut::<OrbitLineGizmos>();
    config.line.width = 1.0;
}

fn draw_orbit_circles(
    mut gizmos: Gizmos<OrbitLineGizmos>,
    orbits: Query<(&ConstantOrbit, &InSector), With<IsEntitySelected>>,
    sectors: Query<&Sector>,
) {
    for (orbit, in_sector) in orbits.iter() {
        let center = sectors.get(in_sector.into()).unwrap().world_pos;

        gizmos
            .circle_2d(
                center,
                orbit.polar_coordinates.radial_distance,
                constants::colors::ORBIT_PREVIEW_COLOR,
            )
            .resolution(32 + (orbit.polar_coordinates.radial_distance / 5.0) as u32);
    }
}

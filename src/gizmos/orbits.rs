use bevy::prelude::{GizmoConfigGroup, GizmoConfigStore, Gizmos, Query, Reflect, ResMut, With};

use crate::components::{ConstantOrbit, InSector, Sector};
use crate::entity_selection::IsEntitySelected;
use common::constants;

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct OrbitLineGizmos;

pub fn configure(mut config_store: ResMut<GizmoConfigStore>) {
    let (config, _) = config_store.config_mut::<OrbitLineGizmos>();
    config.line.width = 1.0;
}

pub fn draw_orbit_circles(
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

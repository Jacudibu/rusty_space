use bevy::prelude::{GizmoConfigGroup, GizmoConfigStore, Gizmos, Query, Reflect, ResMut, With};

use crate::components::{ConstantOrbit, InSector, SectorComponent};
use crate::entity_selection::Selected;

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct OrbitLineGizmos;

pub fn configure(mut config_store: ResMut<GizmoConfigStore>) {
    let (config, _) = config_store.config_mut::<OrbitLineGizmos>();
    config.line_width = 1.0;
}

pub fn draw_orbit_circles(
    mut gizmos: Gizmos<OrbitLineGizmos>,
    orbits: Query<(&ConstantOrbit, &InSector), With<Selected>>,
    sectors: Query<&SectorComponent>,
) {
    for (orbit, in_sector) in orbits.iter() {
        let center = sectors.get(in_sector.into()).unwrap().world_pos;

        gizmos
            .circle_2d(center, orbit.radius, bevy::color::palettes::css::INDIGO)
            .resolution(32 + (orbit.radius / 5.0) as u32);
    }
}

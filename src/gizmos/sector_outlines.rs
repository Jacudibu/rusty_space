use bevy::prelude::{GizmoConfigGroup, Gizmos, Query, Reflect, Res, With};

use crate::components::Sector;
use crate::map_layout::MapLayout;
use crate::simulation::transform::simulation_transform::SimulationTransform;

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct SectorOutlineGizmos;

pub fn draw_sector_outlines(
    mut gizmos: Gizmos<SectorOutlineGizmos>,
    layout: Res<MapLayout>,
    sectors: Query<&SimulationTransform, With<Sector>>,
) {
    for transform in sectors.iter() {
        for edge in layout.hex_edge_vertices {
            gizmos.line_2d(
                edge[0] + transform.translation,
                edge[1] + transform.translation,
                bevy::color::palettes::css::YELLOW,
            );
        }
    }
}

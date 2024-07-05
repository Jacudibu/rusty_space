use bevy::prelude::{
    GizmoConfigGroup, Gizmos, Query, Reflect, Res, Transform, ViewVisibility, With,
};

use crate::components::Sector;
use crate::map_layout::MapLayout;

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct SectorOutlineGizmos;

pub fn draw_sector_outlines(
    mut gizmos: Gizmos<SectorOutlineGizmos>,
    layout: Res<MapLayout>,
    sectors: Query<(&ViewVisibility, &Transform), With<Sector>>,
) {
    for (_, transform) in sectors.iter().filter(|(&_visibility, _)| true /*TODO*/) {
        let pos = transform.translation.truncate();
        for edge in layout.hex_edge_vertices {
            gizmos.line_2d(
                edge[0] + pos,
                edge[1] + pos,
                bevy::color::palettes::css::YELLOW,
            );
        }
    }
}

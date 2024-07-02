use crate::sectors::map_layout::MapLayout;
use crate::sectors::sector::SectorComponent;
use bevy::prelude::{GizmoConfigGroup, Gizmos, Query, Reflect, Res, Vec2, ViewVisibility};
use hexx::{Hex, HexLayout};

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct SectorOutlineGizmos;

pub fn draw_sector_outlines(
    mut gizmos: Gizmos<SectorOutlineGizmos>,
    layout: Res<MapLayout>,
    sectors: Query<(&SectorComponent, &ViewVisibility)>,
) {
    let mut offset_layout = layout.hex_layout.clone();
    offset_layout.hex_size = hexx::Vec2::splat(-5.0);
    let offset = Hex::ZERO
        .all_vertices()
        .map(|vertex| offset_layout.vertex_coordinates(vertex));

    for (sector, _) in sectors.iter().filter(|(_, &visibility)| true /*TODO*/) {
        let vertices = sector_border_vertices(sector.coordinate, &layout.hex_layout, offset);

        gizmos.line_2d(vertices[0], vertices[1], bevy::color::palettes::css::YELLOW);
        gizmos.line_2d(vertices[1], vertices[2], bevy::color::palettes::css::YELLOW);
        gizmos.line_2d(vertices[2], vertices[3], bevy::color::palettes::css::YELLOW);
        gizmos.line_2d(vertices[3], vertices[4], bevy::color::palettes::css::YELLOW);
        gizmos.line_2d(vertices[4], vertices[5], bevy::color::palettes::css::YELLOW);
        gizmos.line_2d(vertices[5], vertices[0], bevy::color::palettes::css::YELLOW);
    }
}

fn sector_border_vertices(hex: Hex, layout: &HexLayout, offset: [hexx::Vec2; 6]) -> [Vec2; 6] {
    let mut result = [Vec2::ZERO; 6];

    let grid_vertices = hex.all_vertices();
    for i in 0..6 {
        let coordinates = layout.vertex_coordinates(grid_vertices[i]);
        // TODO: Once hexx::glam is updated we can just do coordinates + offset
        result[i] = Vec2::new(coordinates.x + offset[i].x, coordinates.y + offset[i].y);
    }

    result
}

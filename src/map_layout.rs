use crate::constants;
use bevy::math::Vec2;
use bevy::prelude::Resource;
use hexx::{Hex, HexLayout, HexOrientation};

#[derive(Resource)]
pub struct MapLayout {
    pub hex_layout: HexLayout,
    pub hex_edge_vertices: [[Vec2; 2]; 6],
}

impl Default for MapLayout {
    fn default() -> Self {
        let layout = HexLayout {
            orientation: HexOrientation::Pointy,
            hex_size: hexx::Vec2::splat(constants::SECTOR_SIZE),
            invert_y: true,
            ..Default::default()
        };

        let mut outline_layout = layout.clone();
        outline_layout.hex_size *= constants::SECTOR_AREA_PERCENTAGE;

        let hex_edge_vertices = outline_layout
            .all_edge_coordinates(Hex::ZERO)
            // TODO glam hexx update 0.14 can skip that silly map conversion...
            .map(|x| x.map(|x| Vec2::new(x.x, x.y)));

        MapLayout {
            hex_layout: HexLayout {
                orientation: HexOrientation::Pointy,
                hex_size: hexx::Vec2::splat(constants::SECTOR_SIZE),
                invert_y: true,
                ..Default::default()
            },
            hex_edge_vertices,
        }
    }
}

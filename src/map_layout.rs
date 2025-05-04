use bevy::math::Vec2;
use bevy::prelude::Resource;
use common::constants;
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
            scale: Vec2::splat(constants::SECTOR_SIZE),
            origin: Vec2::ZERO,
        };

        let mut outline_layout = layout.clone();
        outline_layout.scale *= constants::SECTOR_AREA_PERCENTAGE;

        let hex_edge_vertices = outline_layout.all_edge_coordinates(Hex::ZERO);

        MapLayout {
            hex_layout: HexLayout {
                orientation: HexOrientation::Pointy,
                scale: Vec2::splat(constants::SECTOR_SIZE),
                ..Default::default()
            },
            hex_edge_vertices,
        }
    }
}

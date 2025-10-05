use crate::constants;
use crate::hexx_convert::{HexxConvert, HexxConvertBack};
use bevy::prelude::{Resource, Vec2};
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
            scale: Vec2::splat(constants::SECTOR_SIZE).convert(),
            origin: Vec2::ZERO.convert(),
        };

        let mut outline_layout = layout.clone();
        outline_layout.scale *= constants::SECTOR_AREA_PERCENTAGE;

        let hex_edge_vertices = outline_layout
            .all_edge_coordinates(Hex::ZERO)
            .map(|x| x.map(|x| x.convert()));

        MapLayout {
            hex_layout: HexLayout {
                orientation: HexOrientation::Pointy,
                scale: hexx::Vec2::splat(constants::SECTOR_SIZE),
                ..Default::default()
            },
            hex_edge_vertices,
        }
    }
}

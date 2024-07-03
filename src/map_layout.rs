use crate::constants;
use bevy::prelude::Resource;
use hexx::{HexLayout, HexOrientation};

#[derive(Resource)]
pub struct MapLayout {
    pub hex_layout: HexLayout,
}

impl Default for MapLayout {
    fn default() -> Self {
        MapLayout {
            hex_layout: HexLayout {
                orientation: HexOrientation::Pointy,
                hex_size: hexx::Vec2::splat(constants::SECTOR_SIZE),
                invert_y: true,
                ..Default::default()
            },
        }
    }
}

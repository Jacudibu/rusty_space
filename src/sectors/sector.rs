use crate::sectors::sector_data::SectorData;
use crate::utils::KeyValueResource;
use bevy::prelude::{Commands, Component, SpatialBundle, Transform, Vec2, Vec3};
use hexx::{Hex, HexLayout};

pub type SectorId = Hex;

/// Marker Component for Sectors
#[derive(Component)]
pub struct SectorComponent {
    pub coordinate: Hex,
}

pub type AllSectors = KeyValueResource<SectorId, SectorData>;

pub fn spawn_sector(
    commands: &mut Commands,
    layout: &HexLayout,
    coordinate: Hex,
    all_sectors: &mut AllSectors,
) {
    let position = layout.hex_to_world_pos(coordinate);
    // TODO: remove this once hexx is updated to same glam crate as bevy 0.14
    let position = Vec2::new(position.x, position.y);

    let entity = commands
        .spawn((
            SectorComponent { coordinate },
            SpatialBundle {
                transform: Transform {
                    translation: Vec3::new(position.x, position.y, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .id();

    all_sectors.insert(coordinate, SectorData::new(coordinate, entity, position));
}

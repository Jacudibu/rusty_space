use crate::components::Sector;
use crate::utils::SectorEntity;
use bevy::core::Name;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Commands, SpatialBundle, Transform};
use hexx::{Hex, HexLayout};

pub fn spawn_sector(commands: &mut Commands, layout: &HexLayout, coordinate: Hex) -> SectorEntity {
    let position = layout.hex_to_world_pos(coordinate);
    // TODO: remove this once hexx is updated to same glam crate as bevy 0.14
    let position = Vec2::new(position.x, position.y);

    let entity = commands
        .spawn((
            Name::new(format!("[{},{}]", coordinate.x, coordinate.y)),
            Sector::new(coordinate, position),
            SpatialBundle {
                transform: Transform {
                    translation: Vec3::new(position.x, position.y, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .id();

    SectorEntity::from(entity)
}

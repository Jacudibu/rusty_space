use crate::utils::data_resource::KeyValueResource;
use bevy::prelude::{Commands, Component, Entity, SpatialBundle, Transform, Vec2, Vec3};
use hexx::{Hex, HexLayout};

pub struct SectorData {
    pub coordinate: Hex,
    pub entity: Entity,
    pub gates: Vec<(Entity, Hex)>,
    pub ships: Vec<Entity>,
    pub stations: Vec<Entity>,
}

impl SectorData {
    pub fn new(coordinate: Hex, entity: Entity) -> Self {
        SectorData {
            coordinate,
            entity,
            gates: Vec::new(),
            ships: Vec::new(),
            stations: Vec::new(),
        }
    }
}

#[derive(Component)]
pub struct SectorComponent {
    pub coordinate: Hex,
}

pub type AllSectors = KeyValueResource<Hex, SectorData>;

pub fn spawn_sector(
    commands: &mut Commands,
    layout: &HexLayout,
    coordinate: Hex,
    all_sectors: &mut AllSectors,
) {
    let position = layout.hex_to_world_pos(coordinate);

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

    all_sectors.insert(coordinate, SectorData::new(coordinate, entity));
}

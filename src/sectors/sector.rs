use crate::utils::KeyValueResource;
use bevy::prelude::{Commands, Component, Entity, SpatialBundle, Transform, Vec2, Vec3};
use bevy::utils::HashMap;
use hexx::{Hex, HexLayout};

pub struct SectorData {
    pub coordinate: Hex,
    pub entity: Entity,
    pub world_pos: Vec2,
    pub gates: HashMap<Hex, Entity>,
    pub ships: Vec<Entity>,
    pub stations: Vec<Entity>,
}

impl SectorData {
    pub fn new(coordinate: Hex, entity: Entity, world_pos: Vec2) -> Self {
        SectorData {
            coordinate,
            entity,
            world_pos,
            gates: HashMap::new(),
            ships: Vec::new(),
            stations: Vec::new(),
        }
    }
}

/// Marker Component for Sectors
#[derive(Component)]
pub struct SectorComponent {
    pub coordinate: Hex,
}

/// Component for entities inside sectors
#[derive(Component, PartialEq, Eq)]
pub struct InSector {
    pub sector: Hex,
}

impl From<&SectorData> for InSector {
    fn from(value: &SectorData) -> Self {
        Self {
            sector: value.coordinate,
        }
    }
}

impl From<Hex> for InSector {
    fn from(value: Hex) -> Self {
        Self { sector: value }
    }
}

pub type AllSectors = KeyValueResource<Hex, SectorData>;

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

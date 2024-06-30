use bevy::prelude::{Commands, Component, Entity, Resource, SpatialBundle, Transform, Vec2, Vec3};
use bevy::utils::HashMap;
use hexx::{Hex, HexLayout};

pub struct Sector {
    pub coordinates: Hex,
    pub gates: Vec<Entity>,
    pub ships: Vec<Entity>,
    pub stations: Vec<Entity>,
}

pub struct GateData {
    pub entity: Entity,
    pub position: Vec2,
    pub target_sector: Entity,
    pub target_sector_coordinate: Hex,
}

#[derive(Component)]
pub struct SectorComponent {
    pub coordinate: Hex,
}

#[derive(Resource)]
pub struct SectorData {
    sectors: HashMap<Hex, Entity>,
}

pub fn spawn_sector(commands: &mut Commands, layout: &HexLayout, coordinate: Hex) {
    let position = layout.hex_to_world_pos(coordinate);

    commands.spawn((
        SectorComponent { coordinate },
        SpatialBundle {
            transform: Transform {
                translation: Vec3::new(position.x, position.y, 0.0),
                ..Default::default()
            },
            ..Default::default()
        },
    ));
}

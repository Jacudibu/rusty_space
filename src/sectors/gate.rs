use crate::sectors::sector::AllSectors;
use crate::utils::data_resource::KeyValueResource;
use crate::{constants, SpriteHandles};
use bevy::prelude::{BuildChildren, Commands, Component, Entity, SpriteBundle, Transform, Vec2};
use hexx::Hex;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct GateId {
    pub from: Hex,
    pub to: Hex,
}

impl GateId {
    pub fn invert(&self) -> Self {
        GateId {
            from: self.to,
            to: self.from,
        }
    }
}

pub struct GateData {
    pub id: GateId,
    pub entity: Entity,
    pub position: Vec2,
}

#[derive(Component)]
pub struct GateComponent {
    pub id: GateId,
}

pub type AllGates = KeyValueResource<GateId, GateData>;

// TODO: Alternatively or maybe also:
pub struct DoubleGateData {
    id: (Hex, Hex),
    entities: (Entity, Entity),
    positions: (Vec2, Vec2),
}

// Might be more idiomatic to some degree as we don't have any overhead with ID generation,
// but hashing in a way where we don't need two maps might be a bit annoying...?
// Could try a custom data type instead of the hex tuple
// GateConnections would ideally also use a (Hex, Hex) id, but they could be spawned together with the two gates.
pub type AllDoubleGates = KeyValueResource<(Hex, Hex), GateData>;

pub fn spawn_gates(
    commands: &mut Commands,
    sprites: &SpriteHandles,
    a: SectorPosition,
    b: SectorPosition,
    all_sectors: &mut AllSectors,
    all_gates: &mut AllGates,
) {
    spawn_gate(commands, sprites, &a, &b, all_sectors, all_gates);
    spawn_gate(commands, sprites, &b, &a, all_sectors, all_gates);
}

fn spawn_gate(
    commands: &mut Commands,
    sprites: &SpriteHandles,
    pos: &SectorPosition,
    other: &SectorPosition,
    all_sectors: &mut AllSectors,
    all_gates: &mut AllGates,
) {
    let id = GateId {
        from: pos.sector,
        to: other.sector,
    };
    let sector = all_sectors.get_mut(&pos.sector).unwrap();
    let entity = commands
        .spawn((
            GateComponent { id },
            SpriteBundle {
                transform: Transform::from_translation(pos.position.extend(constants::GATE_LAYER)),
                texture: sprites.gate.clone(),
                ..Default::default()
            },
        ))
        .set_parent(sector.entity)
        .id();

    sector.gates.push((entity, other.sector));

    all_gates.insert(
        id,
        GateData {
            id,
            entity,
            position: pos.position,
        },
    );
}

pub struct SectorPosition {
    pub sector: Hex,
    pub position: Vec2,
}

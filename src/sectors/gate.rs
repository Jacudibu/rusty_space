use crate::sectors::gate_connection::SetupGateConnectionEvent;
use crate::sectors::sector::AllSectors;
use crate::utils::KeyValueResource;
use crate::utils::SectorPosition;
use crate::{constants, SpriteHandles};
use bevy::prelude::{
    BuildChildren, Commands, Component, Entity, EventWriter, SpriteBundle, Transform, Vec2,
};
use hexx::Hex;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct GateId {
    pub from: Hex,
    pub to: Hex,
}

impl GateId {
    /// Returns the ID for the connected gate.
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

pub fn spawn_gates(
    commands: &mut Commands,
    sprites: &SpriteHandles,
    a: SectorPosition,
    b: SectorPosition,
    all_sectors: &mut AllSectors,
    all_gates: &mut AllGates,
    gate_connection_events: &mut EventWriter<SetupGateConnectionEvent>,
) {
    let from = spawn_gate(commands, sprites, &a, &b, all_sectors, all_gates);
    let to = spawn_gate(commands, sprites, &b, &a, all_sectors, all_gates);
    gate_connection_events.send(SetupGateConnectionEvent { from, to });
}

fn spawn_gate(
    commands: &mut Commands,
    sprites: &SpriteHandles,
    pos: &SectorPosition,
    other: &SectorPosition,
    all_sectors: &mut AllSectors,
    all_gates: &mut AllGates,
) -> Entity {
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

    sector.gates.insert(other.sector, entity);

    all_gates.insert(
        id,
        GateData {
            id,
            entity,
            position: pos.position,
        },
    );

    entity
}

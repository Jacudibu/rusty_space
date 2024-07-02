use crate::components::SelectableEntity;
use crate::sectors::gate_connection::SetupGateConnectionEvent;
use crate::sectors::sector::AllSectors;
use crate::utils::KeyValueResource;
use crate::utils::SectorPosition;
use crate::{constants, SpriteHandles};
use bevy::prelude::{
    BuildChildren, Commands, Component, Entity, EventWriter, Name, SpriteBundle, Transform, Vec2,
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
    pub world_position: Vec2,
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
    let position = sector.world_pos + pos.local_position;
    let entity = commands
        .spawn((
            GateComponent { id },
            Name::new(format!(
                "Gate [{},{}] -> [{},{}]",
                pos.sector.x, pos.sector.y, other.sector.x, other.sector.y
            )),
            SelectableEntity::Gate,
            SpriteBundle {
                transform: Transform::from_translation(position.extend(constants::GATE_LAYER)),
                texture: sprites.gate.clone(),
                ..Default::default()
            },
        ))
        .id();

    sector.gates.insert(other.sector, entity);

    all_gates.insert(
        id,
        GateData {
            id,
            entity,
            world_position: position,
        },
    );

    entity
}

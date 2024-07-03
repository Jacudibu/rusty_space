use crate::components::SelectableEntity;
use crate::sectors::gate_connection::SetupGateConnectionEvent;
use crate::sectors::Sector;
use crate::utils::KeyValueResource;
use crate::utils::SectorPosition;
use crate::{constants, SpriteHandles};
use bevy::prelude::{
    Commands, Component, Entity, EventWriter, Name, Query, SpriteBundle, Transform, Vec2,
};

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct GatePair {
    pub from: Entity,
    pub to: Entity,
}

impl GatePair {
    /// Returns the ID for the connected gate.
    pub fn invert(&self) -> Self {
        GatePair {
            from: self.to,
            to: self.from,
        }
    }
}

pub struct GateData {
    pub id: GatePair,
    pub entity: Entity,
    pub world_position: Vec2,
}

#[derive(Component)]
pub struct GateComponent {
    pub id: GatePair,
}

pub type AllGates = KeyValueResource<GatePair, GateData>;

pub fn spawn_gates(
    commands: &mut Commands,
    sector_query: &mut Query<&mut Sector>,
    sprites: &SpriteHandles,
    from_pos: SectorPosition,
    to_pos: SectorPosition,
    all_gates: &mut AllGates,
    gate_connection_events: &mut EventWriter<SetupGateConnectionEvent>,
) {
    let [mut from_sector, mut to_sector] = sector_query
        .get_many_mut([from_pos.sector, to_pos.sector])
        .unwrap();

    let from_gate = spawn_gate(
        commands,
        sprites,
        &from_pos,
        &to_pos,
        &mut from_sector,
        &to_sector,
        all_gates,
    );
    let to_gate = spawn_gate(
        commands,
        sprites,
        &to_pos,
        &from_pos,
        &mut to_sector,
        &from_sector,
        all_gates,
    );

    from_sector.add_gate(commands, from_pos.sector, from_gate, to_pos.sector, to_gate);
    to_sector.add_gate(commands, to_pos.sector, to_gate, from_pos.sector, from_gate);

    gate_connection_events.send(SetupGateConnectionEvent {
        from: from_gate,
        to: to_gate,
    });
}

fn spawn_gate(
    commands: &mut Commands,
    sprites: &SpriteHandles,
    pos: &SectorPosition,
    other: &SectorPosition,
    from: &mut Sector,
    to: &Sector,
    all_gates: &mut AllGates,
) -> Entity {
    let id = GatePair {
        from: pos.sector,
        to: other.sector,
    };
    let position = from.world_pos + pos.local_position;
    let entity = commands
        .spawn((
            GateComponent { id },
            Name::new(format!(
                "Gate [{},{}] -> [{},{}]",
                from.coordinate.x, from.coordinate.y, to.coordinate.x, to.coordinate.y
            )),
            SelectableEntity::Gate,
            SpriteBundle {
                transform: Transform::from_translation(position.extend(constants::GATE_LAYER)),
                texture: sprites.gate.clone(),
                ..Default::default()
            },
        ))
        .id();

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

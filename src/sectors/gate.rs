use crate::components::SelectableEntity;
use crate::sectors::gate_connection::SetupGateConnectionEvent;
use crate::sectors::{GateEntity, Sector, SectorEntity};
use crate::utils::SectorPosition;
use crate::{constants, SpriteHandles};
use bevy::prelude::{
    Commands, Component, CubicCurve, EventWriter, Name, Query, SpriteBundle, Transform, Vec3,
};

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct GateConnectedSectors {
    pub from: SectorEntity,
    pub to: SectorEntity,
}

impl GateConnectedSectors {
    /// Returns the ID for the connected gate.
    pub fn invert(&self) -> Self {
        GateConnectedSectors {
            from: self.to,
            to: self.from,
        }
    }
}

#[derive(Component)]
pub struct GateComponent {
    pub connected_sectors: GateConnectedSectors,
}

#[derive(Component)]
pub struct GateTransitCurve {
    pub transit_curve: CubicCurve<Vec3>,
}

pub fn spawn_gates(
    commands: &mut Commands,
    sector_query: &mut Query<&mut Sector>,
    sprites: &SpriteHandles,
    from_pos: SectorPosition,
    to_pos: SectorPosition,
    gate_connection_events: &mut EventWriter<SetupGateConnectionEvent>,
) {
    let [mut from_sector, mut to_sector] = sector_query
        .get_many_mut([from_pos.sector.get(), to_pos.sector.get()])
        .unwrap();

    let from_gate = spawn_gate(
        commands,
        sprites,
        &from_pos,
        &to_pos,
        &mut from_sector,
        &to_sector,
    );
    let to_gate = spawn_gate(
        commands,
        sprites,
        &to_pos,
        &from_pos,
        &mut to_sector,
        &from_sector,
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
) -> GateEntity {
    let position = from.world_pos + pos.local_position;
    let entity = commands
        .spawn((
            GateComponent {
                connected_sectors: GateConnectedSectors {
                    from: pos.sector,
                    to: other.sector,
                },
            },
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

    let entity = GateEntity::from(entity);
    entity
}

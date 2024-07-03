use crate::components::SelectableEntity;
use crate::sectors::SetupGateConnectionEvent;
use crate::sectors::{GateEntity, Sector};
use crate::utils::SectorPosition;
use crate::{constants, SpriteHandles};
use bevy::core::Name;
use bevy::prelude::{Commands, EventWriter, Query, SpriteBundle, Transform};

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

    let from_gate = spawn_gate(commands, sprites, &from_pos, &mut from_sector, &to_sector);
    let to_gate = spawn_gate(commands, sprites, &to_pos, &mut to_sector, &from_sector);

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
    from: &mut Sector,
    to: &Sector,
) -> GateEntity {
    let position = from.world_pos + pos.local_position;
    let entity = commands
        .spawn((
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

    GateEntity::from(entity)
}

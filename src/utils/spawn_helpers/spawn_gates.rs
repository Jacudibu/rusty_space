use bevy::core::Name;
use bevy::prelude::{Commands, EventWriter, Query, SpriteBundle, Transform};

use crate::components::{Sector, SelectableEntity};
use crate::gizmos::SetupGateConnectionEvent;
use crate::persistence::GateIdMap;
use crate::utils::GateEntity;
use crate::utils::SectorPosition;
use crate::{constants, SpriteHandles};

pub fn spawn_gate_pair(
    commands: &mut Commands,
    gate_id_map: &mut GateIdMap,
    sector_query: &mut Query<&mut Sector>,
    sprites: &SpriteHandles,
    from_pos: SectorPosition,
    to_pos: SectorPosition,
    gate_connection_events: &mut EventWriter<SetupGateConnectionEvent>,
) {
    let [mut from_sector, mut to_sector] = sector_query
        .get_many_mut([from_pos.sector.into(), to_pos.sector.into()])
        .unwrap();

    let from_gate = spawn_gate(
        commands,
        gate_id_map,
        sprites,
        &from_pos,
        &mut from_sector,
        &to_sector,
    );
    let to_gate = spawn_gate(
        commands,
        gate_id_map,
        sprites,
        &to_pos,
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
    gate_id_map: &mut GateIdMap,
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

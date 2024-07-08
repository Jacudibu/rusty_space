use crate::components::Sector;
use crate::gizmos::SetupGateConnectionEvent;
use crate::hex_to_sector_entity_map::HexToSectorEntityMap;
use crate::universe_builder::gate_builder::resources::GateSpawnData;
use crate::SpriteHandles;
use bevy::prelude::{Commands, EventWriter, Query, Res};

pub fn spawn_all_gates(
    mut commands: Commands,
    spawn_data: Res<GateSpawnData>,
    sprites: Res<SpriteHandles>,
    mut sectors: Query<&mut Sector>,
    hex_to_sector_entity_map: Res<HexToSectorEntityMap>,
    mut gate_connection_events: EventWriter<SetupGateConnectionEvent>,
) {
    for builder in &spawn_data.gates {
        builder.build(
            &mut commands,
            &sprites,
            &mut sectors,
            &hex_to_sector_entity_map,
            &mut gate_connection_events,
        );
    }

    commands.remove_resource::<GateSpawnData>();
}

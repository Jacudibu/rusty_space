use crate::components::Sector;
use crate::gizmos::SetupGateConnectionEvent;
use crate::hex_to_sector_entity_map::HexToSectorEntityMap;
use crate::universe_builder::local_hex_position::LocalHexPosition;
use crate::utils::spawn_helpers::spawn_gates;
use crate::SpriteHandles;
use bevy::prelude::{Commands, EventWriter, Query};

pub struct GateSpawnDataInstanceBuilder {
    pub from: LocalHexPosition,
    pub to: LocalHexPosition,
}

impl GateSpawnDataInstanceBuilder {
    pub fn build(
        &self,
        commands: &mut Commands,
        sprites: &SpriteHandles,
        sectors: &mut Query<&mut Sector>,
        hex_to_sector_entity_map: &HexToSectorEntityMap,
        gate_connection_events: &mut EventWriter<SetupGateConnectionEvent>,
    ) {
        // TODO: SectorPosition is exclusively used for gate spawning, might be best to remove it
        // TODO: GateConnections could also be spawned in here, no event needed

        spawn_gates(
            commands,
            sectors,
            sprites,
            self.from.to_sector_position(hex_to_sector_entity_map),
            self.to.to_sector_position(hex_to_sector_entity_map),
            gate_connection_events,
        )
    }
}
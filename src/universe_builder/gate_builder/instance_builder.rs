use crate::components::Sector;
use crate::gizmos::SetupGateConnectionEvent;
use crate::hex_to_sector_entity_map::HexToSectorEntityMap;
use crate::utils::spawn_helpers::spawn_gates;
use crate::utils::SectorPosition;
use crate::SpriteHandles;
use bevy::math::Vec2;
use bevy::prelude::{Commands, EventWriter, Query};
use hexx::Hex;

pub struct GateSpawnDataInstanceBuilder {
    pub from: HexPosition,
    pub to: HexPosition,
}

pub struct HexPosition {
    pub sector: Hex,
    pub position: Vec2,
}

impl HexPosition {
    pub fn new(sector: Hex, position: Vec2) -> Self {
        Self { sector, position }
    }

    pub fn to_sector_position(
        &self,
        hex_to_sector_entity_map: &HexToSectorEntityMap,
    ) -> SectorPosition {
        SectorPosition {
            sector: hex_to_sector_entity_map.map[&self.sector],
            local_position: self.position,
        }
    }
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

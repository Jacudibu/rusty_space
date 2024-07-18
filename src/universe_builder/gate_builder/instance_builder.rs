use crate::components::Sector;
use crate::persistence::local_hex_position::LocalHexPosition;
use crate::persistence::{GateIdMap, SectorIdMap};
use crate::utils::spawn_helpers::spawn_gate_pair;
use crate::SpriteHandles;
use bevy::prelude::{Commands, Query};

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
        sector_id_map: &SectorIdMap,
        gate_id_map: &mut GateIdMap,
    ) {
        // TODO: SectorPosition is exclusively used for gate spawning, might be best to remove it
        // TODO: GateConnections could also be spawned in here, no event needed

        spawn_gate_pair(
            commands,
            gate_id_map,
            sectors,
            sprites,
            self.from.to_sector_position(sector_id_map),
            self.to.to_sector_position(sector_id_map),
        )
    }
}

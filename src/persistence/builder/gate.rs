use crate::components::Sector;
use crate::persistence::data::v1::*;
use crate::persistence::{GateIdMap, PersistentGateId, SectorIdMap};
use crate::universe_builder::LocalHexPosition;
use crate::utils::spawn_helpers::spawn_gate_pair_with_ids;
use crate::SpriteHandles;
use bevy::prelude::{Commands, Query, Res};

impl SaveDataCollection<GatePairSaveData> {
    pub fn add(&mut self, from: LocalHexPosition, to: LocalHexPosition) {
        self.data.push(GatePairSaveData {
            from_id: PersistentGateId::next(),
            from_position: from,
            to_id: PersistentGateId::next(),
            to_position: to,
        })
    }

    pub fn spawn_all(
        &self,
        mut commands: Commands,
        sprites: Res<SpriteHandles>,
        mut sectors: Query<&mut Sector>,
        sector_id_map_entity_map: Res<SectorIdMap>,
    ) {
        let mut gate_id_map = GateIdMap::new();
        for builder in &self.data {
            builder.build(
                &mut commands,
                &sprites,
                &mut sectors,
                &sector_id_map_entity_map,
                &mut gate_id_map,
            );
        }

        commands.insert_resource(gate_id_map);
    }
}

impl GatePairSaveData {
    pub fn build(
        &self,
        commands: &mut Commands,
        sprites: &SpriteHandles,
        sectors: &mut Query<&mut Sector>,
        sector_id_map_entity_map: &SectorIdMap,
        gate_id_map: &mut GateIdMap,
    ) {
        // TODO: SectorPosition is exclusively used for gate spawning, might be best to remove it

        spawn_gate_pair_with_ids(
            commands,
            gate_id_map,
            sectors,
            sprites,
            self.from_id,
            self.from_position
                .to_sector_position(sector_id_map_entity_map),
            self.to_id,
            self.to_position
                .to_sector_position(sector_id_map_entity_map),
        )
    }
}

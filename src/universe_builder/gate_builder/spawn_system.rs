use crate::components::Sector;
use crate::persistence::{GateIdMap, SectorIdMap};
use crate::universe_builder::gate_builder::data_resource::GateSpawnData;
use crate::SpriteHandles;
use bevy::prelude::{Commands, Query, Res};

pub fn spawn_all_gates(
    mut commands: Commands,
    spawn_data: Res<GateSpawnData>,
    sprites: Res<SpriteHandles>,
    mut sectors: Query<&mut Sector>,
    sector_id_map: Res<SectorIdMap>,
) {
    let mut gate_id_map = GateIdMap::new();
    for builder in &spawn_data.gates {
        builder.build(
            &mut commands,
            &sprites,
            &mut sectors,
            &sector_id_map,
            &mut gate_id_map,
        );
    }

    commands.remove_resource::<GateSpawnData>();
    commands.insert_resource(gate_id_map);
}

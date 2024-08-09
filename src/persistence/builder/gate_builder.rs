use crate::components::{Sector, SectorStarComponent, Star};
use crate::persistence::data::v1::*;
use crate::persistence::local_hex_position::LocalHexPosition;
use crate::persistence::{GateIdMap, PersistentGateId, SectorIdMap};
use crate::utils::spawn_helpers::spawn_gate_pair;
use crate::SpriteHandles;
use bevy::ecs::system::SystemParam;
use bevy::prelude::{Commands, Query, Res};

#[derive(SystemParam)]
pub struct Args<'w, 's> {
    commands: Commands<'w, 's>,
    sprites: Res<'w, SpriteHandles>,
    sectors: Query<'w, 's, (&'static mut Sector, Option<&'static SectorStarComponent>)>,
    stars: Query<'w, 's, &'static Star>,

    sector_id_map: Res<'w, SectorIdMap>,
}

type SaveData = SaveDataCollection<GatePairSaveData>;

pub fn spawn_all(data: Res<SaveData>, mut args: Args) {
    let mut gate_id_map = GateIdMap::new();
    for builder in &data.data {
        builder.build(&mut args, &mut gate_id_map);
    }

    args.commands.remove_resource::<SaveData>();
    args.commands.insert_resource(gate_id_map);
}

impl SaveData {
    pub fn add(&mut self, from: LocalHexPosition, to: LocalHexPosition) -> &mut GatePairSaveData {
        self.data.push(GatePairSaveData {
            from_id: PersistentGateId::next(),
            from_position: from,
            to_id: PersistentGateId::next(),
            to_position: to,
        });
        self.data.last_mut().unwrap()
    }
}

impl GatePairSaveData {
    pub fn build(&self, args: &mut Args, gate_id_map: &mut GateIdMap) {
        // TODO: SectorPosition is exclusively used for gate spawning, might be best to remove it

        spawn_gate_pair(
            &mut args.commands,
            gate_id_map,
            &mut args.sectors,
            &args.stars,
            &args.sprites,
            self.from_id,
            self.from_position.to_sector_position(&args.sector_id_map),
            self.to_id,
            self.to_position.to_sector_position(&args.sector_id_map),
        )
    }
}

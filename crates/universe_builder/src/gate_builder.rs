use bevy::ecs::system::SystemParam;
use bevy::prelude::{Commands, Deref, DerefMut, Query, Res};
use common::components::celestials::Celestial;
use common::components::{Sector, SectorWithCelestials};
use common::types::entity_id_map::{GateIdMap, SectorIdMap};
use common::types::local_hex_position::LocalHexPosition;
use common::types::persistent_entity_id::PersistentGateId;
use common::types::sprite_handles::SpriteHandles;
use entity_spawners::spawn_gates::spawn_gate_pair;
use persistence::data::{GatePairSaveData, SaveDataCollection};

#[derive(SystemParam)]
pub struct Args<'w, 's> {
    commands: Commands<'w, 's>,
    sprites: Res<'w, SpriteHandles>,
    sectors: Query<'w, 's, (&'static mut Sector, Option<&'static SectorWithCelestials>)>,
    celestials: Query<'w, 's, &'static Celestial>,

    sector_id_map: Res<'w, SectorIdMap>,
}

#[derive(Deref, DerefMut, Default)]
pub struct GatePairBuilder {
    data: Vec<GatePairSaveData>,
}

impl GatePairBuilder {
    pub fn add(&mut self, from: LocalHexPosition, to: LocalHexPosition) -> &mut GatePairSaveData {
        self.data.push(GatePairSaveData {
            from_id: PersistentGateId::next(),
            from_position: from,
            to_id: PersistentGateId::next(),
            to_position: to,
        });
        self.data.last_mut().unwrap()
    }

    pub fn build(self) -> SaveDataCollection<GatePairSaveData> {
        SaveDataCollection { data: self.data }
    }
}

pub fn spawn_all(data: Res<SaveDataCollection<GatePairSaveData>>, mut args: Args) {
    let mut gate_id_map = GateIdMap::new();

    for data in &data.data {
        spawn_gate_pair(
            &mut args.commands,
            &mut gate_id_map,
            &mut args.sectors,
            &args.sprites,
            data.from_id,
            data.from_position.to_sector_position(&args.sector_id_map),
            data.to_id,
            data.to_position.to_sector_position(&args.sector_id_map),
        )
    }

    args.commands
        .remove_resource::<SaveDataCollection<GatePairSaveData>>();
    args.commands.insert_resource(gate_id_map);
}

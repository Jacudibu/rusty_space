use bevy::prelude::{Deref, DerefMut};
use common::types::local_hex_position::LocalHexPosition;
use common::types::persistent_entity_id::PersistentGateId;
use persistence::data::{GatePairSaveData, SaveDataCollection};

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

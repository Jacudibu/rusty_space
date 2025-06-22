use bevy::prelude::{Deref, DerefMut};
use bevy::reflect::erased_serde::__private::serde::{Deserialize, Serialize};
use common::game_data::ItemId;
use common::session_data::ShipConfigId;
use common::types::local_hex_position::LocalHexPosition;
use common::types::persistent_entity_id::PersistentShipId;
use persistence::data::{
    AutoMineStateSaveData, InventorySaveData, SaveDataCollection, ShipBehaviorSaveData,
    ShipSaveData,
};

#[derive(Deref, DerefMut, Default)]
pub struct ShipBuilder {
    pub data: Vec<ShipSaveData>,
}

impl ShipBuilder {
    pub fn add(
        &mut self,
        config_id: ShipConfigId,
        position: LocalHexPosition,
        rotation: f32,
        name: impl Into<String>,
        behavior: ShipBehaviorSaveData,
    ) -> &mut ShipSaveData {
        self.push(ShipSaveData {
            id: PersistentShipId::next(),
            config_id,
            name: name.into(),
            position,
            rotation_degrees: rotation,
            behavior,
            forward_velocity: 0.0,
            angular_velocity: 0.0,
            task_queue: Vec::new(), // TODO
            inventory: InventorySaveData { items: Vec::new() },
        });
        self.data.last_mut().unwrap()
    }

    pub fn build(self) -> SaveDataCollection<ShipSaveData> {
        SaveDataCollection { data: self.data }
    }
}

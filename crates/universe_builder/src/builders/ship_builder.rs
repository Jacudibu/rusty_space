use bevy::prelude::{Deref, DerefMut};
use common::session_data::ShipConfigId;
use common::types::auto_mine_state::AutoMineState;
use common::types::behavior_builder::BehaviorBuilder;
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
        name: String,
        behavior: ShipBehaviorSaveData,
    ) -> &mut ShipSaveData {
        self.push(ShipSaveData {
            id: PersistentShipId::next(),
            config_id,
            name,
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

pub fn convert_behavior_save_data_to_builder_data(value: ShipBehaviorSaveData) -> BehaviorBuilder {
    match value {
        ShipBehaviorSaveData::AutoTrade => BehaviorBuilder::AutoTrade,
        ShipBehaviorSaveData::AutoConstruct => BehaviorBuilder::AutoConstruct,
        ShipBehaviorSaveData::AutoMine { mined_ore, state } => BehaviorBuilder::AutoMine {
            mined_ore,
            state: convert_auto_mine_state(state),
        },
        ShipBehaviorSaveData::AutoHarvest {
            harvested_gas,
            state,
        } => BehaviorBuilder::AutoHarvest {
            harvested_gas,
            state: convert_auto_mine_state(state),
        },
    }
}

fn convert_auto_mine_state(state: AutoMineStateSaveData) -> AutoMineState {
    match state {
        AutoMineStateSaveData::Mining => AutoMineState::Mining,
        AutoMineStateSaveData::Trading => AutoMineState::Trading,
    }
}

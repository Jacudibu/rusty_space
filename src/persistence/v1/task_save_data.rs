use crate::game_data::ItemId;
use crate::persistence::{
    AllEntityIdMaps, PersistentAsteroidId, PersistentEntityId, PersistentGateId,
};
use crate::ship_ai::TaskInsideQueue;
use crate::utils::ExchangeWareData;
use hexx::Hex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum TaskSaveData {
    ExchangeWares {
        target: PersistentEntityId,
        data: ExchangeWareSaveData,
    },
    MoveToEntity {
        target: PersistentEntityId,
        stop_at_target: bool,
    },
    UseGate {
        enter_gate: PersistentGateId,
        exit_sector: Hex,
    },
    MineAsteroid {
        target: PersistentAsteroidId,
        reserved: u32,
    },
}

impl TaskSaveData {
    pub fn from(task: &TaskInsideQueue, all_entity_id_maps: &AllEntityIdMaps) -> Self {
        match task {
            TaskInsideQueue::ExchangeWares { target, data } => Self::ExchangeWares {
                target: all_entity_id_maps.get_typed_id_unchecked(target),
                data: data.into(),
            },
            TaskInsideQueue::MoveToEntity {
                target,
                stop_at_target,
            } => Self::MoveToEntity {
                target: all_entity_id_maps.get_typed_id_unchecked(target),
                stop_at_target: *stop_at_target,
            },
            TaskInsideQueue::UseGate {
                enter_gate,
                exit_sector,
            } => Self::UseGate {
                enter_gate: all_entity_id_maps.gates.entity_to_id()[enter_gate],
                exit_sector: all_entity_id_maps.sectors.entity_to_id()[exit_sector],
            },
            TaskInsideQueue::MineAsteroid { target, reserved } => Self::MineAsteroid {
                target: all_entity_id_maps.asteroids.entity_to_id()[target],
                reserved: *reserved,
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum ExchangeWareSaveData {
    Buy(ItemId, u32),
    Sell(ItemId, u32),
}

impl From<&ExchangeWareData> for ExchangeWareSaveData {
    fn from(value: &ExchangeWareData) -> Self {
        match value {
            ExchangeWareData::Buy(item, amount) => Self::Buy(*item, *amount),
            ExchangeWareData::Sell(item, amount) => Self::Sell(*item, *amount),
        }
    }
}

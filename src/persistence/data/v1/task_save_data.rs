use crate::game_data::ItemId;
use crate::persistence::{
    PersistentAsteroidId, PersistentEntityId, PersistentGateId, PersistentPlanetId,
};
use hexx::Hex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Clone, Debug, PartialEq))]
pub enum TaskSaveData {
    ExchangeWares {
        target: PersistentEntityId,
        data: ExchangeWareSaveData,
    },
    MoveToEntity {
        target: PersistentEntityId,
        stop_at_target: bool,
        distance_to_target: f32,
    },
    UseGate {
        enter_gate: PersistentGateId,
        exit_sector: Hex,
    },
    MineAsteroid {
        target: PersistentAsteroidId,
        reserved: u32,
    },
    HarvestGas {
        target: PersistentPlanetId,
    },
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Clone, Debug, PartialEq))]
pub enum ExchangeWareSaveData {
    Buy(ItemId, u32),
    Sell(ItemId, u32),
}

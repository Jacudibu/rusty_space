use crate::persistence::PersistentGateId;
use bevy::math::Vec2;
use hexx::Hex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GatePairSaveData {
    pub from_id: PersistentGateId,
    pub from_sector: Hex,
    pub from_position: Vec2,
    pub to_id: PersistentGateId,
    pub to_sector: Hex,
    pub to_position: Vec2,
}

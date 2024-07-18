use crate::persistence::local_hex_position::LocalHexPosition;
use crate::persistence::PersistentGateId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GatePairSaveData {
    pub from_id: PersistentGateId,
    pub from_position: LocalHexPosition,
    pub to_id: PersistentGateId,
    pub to_position: LocalHexPosition,
}

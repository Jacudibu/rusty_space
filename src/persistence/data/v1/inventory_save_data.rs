use crate::game_data::ItemId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct InventorySaveData {
    pub items: Vec<(ItemId, u32)>,
}

use common::game_data::ItemId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Clone, Debug, PartialEq))]
pub struct InventorySaveData {
    pub items: Vec<(ItemId, u32)>,
}

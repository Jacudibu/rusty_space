use crate::components::Inventory;
use crate::game_data::ItemId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct InventorySaveData {
    pub items: Vec<(ItemId, u32)>,
}

impl From<&Inventory> for InventorySaveData {
    fn from(value: &Inventory) -> Self {
        Self {
            items: value
                .inventory()
                .iter()
                .map(|(id, element)| (*id, element.currently_available))
                .collect(),
        }
    }
}

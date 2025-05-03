use crate::components::Inventory;
use crate::persistence::data::v1::*;

impl From<&Inventory> for InventorySaveData {
    fn from(value: &Inventory) -> Self {
        Self {
            items: value
                .inventory()
                .iter()
                .map(|(id, element)| (*id, element.current))
                .collect(),
        }
    }
}

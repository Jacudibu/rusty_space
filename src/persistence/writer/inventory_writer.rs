use crate::components::InventoryComponent;
use crate::persistence::data::v1::*;

impl From<&InventoryComponent> for InventorySaveData {
    fn from(value: &InventoryComponent) -> Self {
        Self {
            items: value
                .inventory()
                .iter()
                .map(|(id, element)| (*id, element.current))
                .collect(),
        }
    }
}

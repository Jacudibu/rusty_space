// use crate::data::InventorySaveData;
// use common::components::Inventory;
//
// impl From<&Inventory> for InventorySaveData {
//     fn from(value: &Inventory) -> Self {
//         Self {
//             items: value
//                 .inventory()
//                 .iter()
//                 .map(|(id, element)| (*id, element.current))
//                 .collect(),
//         }
//     }
// }

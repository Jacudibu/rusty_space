mod item;
mod item_manifest;
mod raw_item;
mod raw_item_manifest;

use leafwing_manifest::identifier::Id;

pub use item::Item;
pub use item_manifest::ItemManifest;
pub type ItemId = Id<Item>;

const DEBUG_ITEM_STRING_A: &str = "item_a";
const DEBUG_ITEM_STRING_B: &str = "item_b";
const DEBUG_ITEM_STRING_C: &str = "item_c";

pub const DEBUG_ITEM_ID_A: ItemId = ItemId::from_name(DEBUG_ITEM_STRING_A);
pub const DEBUG_ITEM_ID_B: ItemId = ItemId::from_name(DEBUG_ITEM_STRING_B);
pub const DEBUG_ITEM_ID_C: ItemId = ItemId::from_name(DEBUG_ITEM_STRING_C);

pub const DEBUG_ITEM_ID_ORE: ItemId = DEBUG_ITEM_ID_A;
pub const DEBUG_ITEM_ID_GAS: ItemId = DEBUG_ITEM_ID_B;

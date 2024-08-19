mod item;
mod item_manifest;
mod raw_item;
mod raw_item_manifest;

use leafwing_manifest::identifier::Id;

pub use item::Item;
pub use item_manifest::ItemManifest;
pub type ItemId = Id<Item>;

const MOCK_ITEM_STRING_A: &str = "item_a";
const MOCK_ITEM_STRING_B: &str = "item_b";
const MOCK_ITEM_STRING_C: &str = "item_c";

pub const MOCK_ITEM_ID_A: ItemId = ItemId::from_name(MOCK_ITEM_STRING_A);
pub const MOCK_ITEM_ID_B: ItemId = ItemId::from_name(MOCK_ITEM_STRING_B);
pub const MOCK_ITEM_ID_C: ItemId = ItemId::from_name(MOCK_ITEM_STRING_C);

pub const MOCK_ITEM_ID_ORE: ItemId = MOCK_ITEM_ID_A;
pub const MOCK_ITEM_ID_GAS: ItemId = MOCK_ITEM_ID_B;

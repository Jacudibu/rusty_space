mod item;
mod item_manifest;
mod raw_item;
mod raw_item_manifest;

use leafwing_manifest::identifier::Id;

use crate::create_id_constants;
pub use item::ItemData;
pub use item_manifest::ItemManifest;

pub type ItemId = Id<ItemData>;

pub const MOCK_ITEM_ORE_ID: ItemId = MOCK_ITEM_A_ID;
pub const MOCK_ITEM_ORE_SILICON_ID: ItemId = MOCK_ITEM_A_ID;
pub const MOCK_ITEM_GAS_ID: ItemId = MOCK_ITEM_B_ID;

create_id_constants!(ItemId, MOCK_ITEM_A, MOCK_ITEM_B, MOCK_ITEM_C);

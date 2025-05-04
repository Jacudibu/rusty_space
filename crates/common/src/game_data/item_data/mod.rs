mod item;
mod item_manifest;
mod raw_item;
mod raw_item_manifest;

use leafwing_manifest::identifier::Id;

use crate::create_id_constants;
pub use item::ItemData;
pub use item_manifest::ItemManifest;

#[cfg(test)]
pub use {raw_item::RawItemData, raw_item_manifest::RawItemManifest};

pub type ItemId = Id<ItemData>;

create_id_constants!(
    ItemId,
    REFINED_METALS_ITEM,
    SILICA_ITEM,
    WAFER_ITEM,
    IRON_ORE_ITEM,
    CRYSTAL_ORE_ITEM,
    HYDROGEN_ITEM
);

use crate::utils::PriceRange;

pub type ItemId = u32;

pub const DEBUG_ITEM_ID: ItemId = 1;

pub struct ItemDefinition {
    pub id: ItemId,
    pub name: String,
    pub price: PriceRange,
}

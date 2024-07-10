use crate::utils::PriceRange;

pub type ItemId = u32;

pub const DEBUG_ITEM_ID_A: ItemId = 1;
pub const DEBUG_ITEM_ID_B: ItemId = 2;
pub const DEBUG_ITEM_ID_C: ItemId = 3;
pub const DEBUG_ITEM_ID_ORE: ItemId = DEBUG_ITEM_ID_A;

pub struct ItemDefinition {
    pub id: ItemId,
    pub icon: String, // TODO: Should be converted into asset handle during parsing
    pub name: String,
    pub price: PriceRange, // TODO: consider autocomputing this depending on ingredient price ranges?
}

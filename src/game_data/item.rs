use crate::utils::PriceRange;
use leafwing_manifest::identifier::Id;

pub type ItemId = Id<ItemDefinition>;

pub const DEBUG_ITEM_ID_A: ItemId = ItemId::from_name("item_a");
pub const DEBUG_ITEM_ID_B: ItemId = ItemId::from_name("item_b");
pub const DEBUG_ITEM_ID_C: ItemId = ItemId::from_name("item_c");
pub const DEBUG_ITEM_ID_ORE: ItemId = DEBUG_ITEM_ID_A;
pub const DEBUG_ITEM_ID_GAS: ItemId = DEBUG_ITEM_ID_B;

pub struct ItemDefinition {
    pub id: ItemId,
    pub icon: String, // TODO: Should be converted into asset handle during parsing
    pub name: String,
    pub price: PriceRange, // TODO: consider autocomputing this depending on ingredient price ranges?
}

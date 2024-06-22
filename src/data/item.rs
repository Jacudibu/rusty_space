pub type ItemId = u32;

pub const DEBUG_ITEM_ID: ItemId = 1;

pub struct ItemDefinition {
    pub id: ItemId,
    pub name: String,
    pub price: PriceRange,
}

pub struct PriceRange {
    pub min: u32,
    pub max: u32,
}

impl PriceRange {
    pub fn new(min: u32, max: u32) -> Self {
        Self { min, max }
    }
}

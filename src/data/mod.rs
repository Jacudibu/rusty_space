mod item;

use crate::data::item::PriceRange;
use bevy::prelude::Resource;
use bevy::utils::HashMap;
pub use item::{ItemDefinition, ItemId, DEBUG_ITEM_ID};

#[derive(Resource)]
pub struct GameData {
    pub items: HashMap<ItemId, ItemDefinition>,
}

impl GameData {
    pub fn mock_data() -> Self {
        let mut items = HashMap::new();
        items.insert(
            DEBUG_ITEM_ID,
            ItemDefinition {
                id: DEBUG_ITEM_ID,
                name: "Item A".into(),
                price: PriceRange::new(5, 10),
            },
        );

        Self { items }
    }
}

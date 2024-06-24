mod item;

use crate::utils::PriceRange;
use bevy::prelude::Resource;
use bevy::utils::HashMap;
pub use item::{ItemDefinition, ItemId, DEBUG_ITEM_ID_A, DEBUG_ITEM_ID_B, DEBUG_ITEM_ID_C};

#[derive(Resource)]
pub struct GameData {
    pub items: HashMap<ItemId, ItemDefinition>,
}

impl GameData {
    pub fn mock_data() -> Self {
        let mut items = HashMap::new();
        items.insert(
            DEBUG_ITEM_ID_A,
            ItemDefinition {
                id: DEBUG_ITEM_ID_A,
                icon: "ui_icons/items/a.png".into(),
                name: "Item A".into(),
                price: PriceRange::new(5, 1000),
            },
        );
        items.insert(
            DEBUG_ITEM_ID_B,
            ItemDefinition {
                id: DEBUG_ITEM_ID_B,
                icon: "ui_icons/items/b.png".into(),
                name: "Item B".into(),
                price: PriceRange::new(5, 1000),
            },
        );
        items.insert(
            DEBUG_ITEM_ID_C,
            ItemDefinition {
                id: DEBUG_ITEM_ID_C,
                icon: "ui_icons/items/c.png".into(),
                name: "Item C".into(),
                price: PriceRange::new(5, 1000),
            },
        );

        Self { items }
    }
}

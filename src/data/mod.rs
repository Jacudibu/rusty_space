mod item;
mod item_recipe;

use crate::utils::PriceRange;
use bevy::prelude::Resource;
use bevy::utils::HashMap;
pub use item::*;
pub use item_recipe::*;

#[derive(Resource)]
pub struct GameData {
    pub items: HashMap<ItemId, ItemDefinition>,
    pub item_recipes: HashMap<RecipeId, ItemRecipe>,
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

        let item_recipes = HashMap::new();

        Self {
            items,
            item_recipes,
        }
    }
}

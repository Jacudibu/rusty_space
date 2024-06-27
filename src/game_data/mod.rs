mod item;
mod item_recipe;
mod production_module;
mod shipyard_module;

use crate::utils::PriceRange;
use bevy::prelude::Resource;
use bevy::utils::HashMap;

pub use {item::*, item_recipe::*, production_module::*, shipyard_module::*};

/// Constant Data which is parsed from files at game start and doesn't change without a restart.
#[derive(Resource)]
pub struct GameData {
    pub items: HashMap<ItemId, ItemDefinition>,
    pub item_recipes: HashMap<RecipeId, ItemRecipe>,
    pub production_modules: HashMap<ProductionModuleId, ProductionModuleDefinition>,
    pub shipyard_modules: HashMap<ShipyardModuleId, ShipyardModuleDefinition>,
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

        let mut item_recipes = HashMap::new();
        item_recipes.insert(
            RECIPE_A_ID,
            ItemRecipe {
                id: RECIPE_A_ID,
                name: "5C -> 10A".into(),
                duration: 10,
                input: vec![ItemRecipeElement {
                    item_id: DEBUG_ITEM_ID_C,
                    amount: 5,
                }],
                output: vec![ItemRecipeElement {
                    item_id: DEBUG_ITEM_ID_A,
                    amount: 10,
                }],
            },
        );
        item_recipes.insert(
            RECIPE_B_ID,
            ItemRecipe {
                id: RECIPE_B_ID,
                name: "5A -> 13B".into(),
                duration: 20,
                input: vec![ItemRecipeElement {
                    item_id: DEBUG_ITEM_ID_A,
                    amount: 5,
                }],
                output: vec![ItemRecipeElement {
                    item_id: DEBUG_ITEM_ID_B,
                    amount: 13,
                }],
            },
        );
        item_recipes.insert(
            RECIPE_C_ID,
            ItemRecipe {
                id: RECIPE_C_ID,
                name: "5B -> 17C".into(),
                duration: 30,
                input: vec![ItemRecipeElement {
                    item_id: DEBUG_ITEM_ID_B,
                    amount: 5,
                }],
                output: vec![ItemRecipeElement {
                    item_id: DEBUG_ITEM_ID_C,
                    amount: 17,
                }],
            },
        );

        let production_modules = HashMap::from([
            (
                PRODUCTION_MODULE_A_ID,
                ProductionModuleDefinition {
                    id: PRODUCTION_MODULE_A_ID,
                    name: "Production Module A".to_string(),
                    available_recipes: vec![RECIPE_A_ID],
                },
            ),
            (
                PRODUCTION_MODULE_B_ID,
                ProductionModuleDefinition {
                    id: PRODUCTION_MODULE_B_ID,
                    name: "Production Module B".to_string(),
                    available_recipes: vec![RECIPE_B_ID],
                },
            ),
            (
                PRODUCTION_MODULE_C_ID,
                ProductionModuleDefinition {
                    id: PRODUCTION_MODULE_C_ID,
                    name: "Production Module C".to_string(),
                    available_recipes: vec![RECIPE_C_ID],
                },
            ),
        ]);

        let shipyard_modules = HashMap::from([(
            SHIPYARD_MODULE_ID,
            ShipyardModuleDefinition {
                id: SHIPYARD_MODULE_ID,
                name: "Debug Shipyard".to_string(),
            },
        )]);

        Self {
            items,
            item_recipes,
            production_modules,
            shipyard_modules,
        }
    }
}

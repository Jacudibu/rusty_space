mod item_data;
mod item_recipe;
mod production_module;
mod shipyard_module;

use bevy::prelude::{FromWorld, Resource, World};
use bevy::utils::HashMap;
pub use {
    item_data::{
        Item, ItemId, ItemManifest, DEBUG_ITEM_ID_A, DEBUG_ITEM_ID_B, DEBUG_ITEM_ID_C,
        DEBUG_ITEM_ID_GAS, DEBUG_ITEM_ID_ORE,
    },
    item_recipe::*,
    production_module::*,
    shipyard_module::*,
};

/// Constant Data which is parsed from files at game start and doesn't change without a restart.
#[derive(Resource)]
pub struct GameData {
    pub items: ItemManifest,
    pub item_recipes: HashMap<RecipeId, ItemRecipe>,
    pub production_modules: HashMap<ProductionModuleId, ProductionModuleDefinition>,
    pub shipyard_modules: HashMap<ShipyardModuleId, ShipyardModuleDefinition>,
}

impl FromWorld for GameData {
    fn from_world(world: &mut World) -> Self {
        let items = ItemManifest::from_mock_data(world);

        let mut item_recipes = HashMap::new();
        item_recipes.insert(
            RECIPE_A_ID,
            ItemRecipe {
                id: RECIPE_A_ID,
                name: "5C -> 10A".into(),
                duration: 10000,
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
                duration: 20000,
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
                duration: 30000,
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

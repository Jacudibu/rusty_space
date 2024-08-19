mod item_data;
mod production_module;
mod recipe_data;
mod shipyard_module;

use crate::game_data::recipe_data::RecipeManifest;
use bevy::prelude::{FromWorld, Resource, World};
use bevy::utils::HashMap;
pub use {
    item_data::{
        Item, ItemId, ItemManifest, DEBUG_ITEM_ID_A, DEBUG_ITEM_ID_B, DEBUG_ITEM_ID_C,
        DEBUG_ITEM_ID_GAS, DEBUG_ITEM_ID_ORE,
    },
    production_module::*,
    recipe_data::{
        Recipe, RecipeElement, RecipeId, MOCK_RECIPE_A_ID, MOCK_RECIPE_B_ID, MOCK_RECIPE_C_ID,
    },
    shipyard_module::*,
};

/// Constant Data which is parsed from files at game start and doesn't change without a restart.
#[derive(Resource)]
pub struct GameData {
    pub items: ItemManifest,
    pub item_recipes: RecipeManifest,
    pub production_modules: HashMap<ProductionModuleId, ProductionModuleDefinition>,
    pub shipyard_modules: HashMap<ShipyardModuleId, ShipyardModuleDefinition>,
}

impl FromWorld for GameData {
    fn from_world(world: &mut World) -> Self {
        let production_modules = HashMap::from([
            (
                PRODUCTION_MODULE_A_ID,
                ProductionModuleDefinition {
                    id: PRODUCTION_MODULE_A_ID,
                    name: "Production Module A".to_string(),
                    available_recipes: vec![MOCK_RECIPE_A_ID],
                },
            ),
            (
                PRODUCTION_MODULE_B_ID,
                ProductionModuleDefinition {
                    id: PRODUCTION_MODULE_B_ID,
                    name: "Production Module B".to_string(),
                    available_recipes: vec![MOCK_RECIPE_B_ID],
                },
            ),
            (
                PRODUCTION_MODULE_C_ID,
                ProductionModuleDefinition {
                    id: PRODUCTION_MODULE_C_ID,
                    name: "Production Module C".to_string(),
                    available_recipes: vec![MOCK_RECIPE_C_ID],
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
            items: ItemManifest::from_mock_data(world),
            item_recipes: RecipeManifest::from_mock_data(world),
            production_modules,
            shipyard_modules,
        }
    }
}

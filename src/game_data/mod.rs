mod item_data;
mod production_data;
mod recipe_data;
mod shipyard_module;

use crate::game_data::recipe_data::RecipeManifest;
use bevy::prelude::{FromWorld, Resource, World};
use bevy::utils::HashMap;
pub use {
    item_data::{
        Item, ItemId, ItemManifest, MOCK_ITEM_ID_A, MOCK_ITEM_ID_B, MOCK_ITEM_ID_C,
        MOCK_ITEM_ID_GAS, MOCK_ITEM_ID_ORE,
    },
    production_data::{
        ProductionModule, ProductionModuleId, ProductionModuleManifest,
        MOCK_PRODUCTION_MODULE_A_ID, MOCK_PRODUCTION_MODULE_B_ID, MOCK_PRODUCTION_MODULE_C_ID,
    },
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
    pub production_modules: ProductionModuleManifest,
    pub shipyard_modules: HashMap<ShipyardModuleId, ShipyardModuleDefinition>,
}

impl FromWorld for GameData {
    fn from_world(world: &mut World) -> Self {
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
            production_modules: ProductionModuleManifest::from_mock_data(world),
            shipyard_modules,
        }
    }
}

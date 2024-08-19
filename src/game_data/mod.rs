mod item_data;
mod production_module_data;
mod recipe_data;
mod shipyard_module_data;

use bevy::ecs::system::SystemParam;
use bevy::prelude::{Res, World};
pub use {
    item_data::{
        ItemData, ItemId, ItemManifest, MOCK_ITEM_ID_A, MOCK_ITEM_ID_B, MOCK_ITEM_ID_C,
        MOCK_ITEM_ID_GAS, MOCK_ITEM_ID_ORE,
    },
    production_module_data::{
        ProductionModuleData, ProductionModuleId, ProductionModuleManifest,
        MOCK_PRODUCTION_MODULE_A_ID, MOCK_PRODUCTION_MODULE_B_ID, MOCK_PRODUCTION_MODULE_C_ID,
    },
    recipe_data::{
        RecipeData, RecipeElement, RecipeId, RecipeManifest, MOCK_RECIPE_A_ID, MOCK_RECIPE_B_ID,
        MOCK_RECIPE_C_ID,
    },
    shipyard_module_data::{
        ShipyardModuleData, ShipyardModuleId, ShipyardModuleManifest, MOCK_SHIPYARD_MODULE_ID,
    },
};

/// A collection of all constant game data.
#[derive(SystemParam)]
pub struct GameData<'w> {
    pub items: Res<'w, ItemManifest>,
    pub item_recipes: Res<'w, RecipeManifest>,
    pub production_modules: Res<'w, ProductionModuleManifest>,
    pub shipyard_modules: Res<'w, ShipyardModuleManifest>,
}

impl<'w> GameData<'w> {
    pub fn initialize_mock_data(world: &mut World) {
        let items = ItemManifest::from_mock_data(world);
        let item_recipes = RecipeManifest::from_mock_data(world);
        let production_modules = ProductionModuleManifest::from_mock_data(world);
        let shipyard_modules = ShipyardModuleManifest::from_mock_data(world);

        world.insert_resource(items);
        world.insert_resource(item_recipes);
        world.insert_resource(production_modules);
        world.insert_resource(shipyard_modules);
    }
}

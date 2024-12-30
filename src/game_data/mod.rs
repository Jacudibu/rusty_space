mod asteroid_data;
mod create_id_constants;
mod from_mock_data;
mod generic_manifest;
mod generic_manifest_without_raw_data;
mod item_data;
mod production_module_data;
mod recipe_data;
mod ship_hull_data;
mod ship_weapon_data;
mod shipyard_module_data;

use crate::game_data::from_mock_data::FromMockData;
use bevy::ecs::system::SystemParam;
use bevy::prelude::{Res, World};

#[allow(unused)]
pub use {
    asteroid_data::{AsteroidDataId, AsteroidManifest, MOCK_ASTEROID_ID, SILICON_ASTEROID_ID},
    item_data::{
        ItemData, ItemId, ItemManifest, MOCK_ITEM_A_ID, MOCK_ITEM_B_ID, MOCK_ITEM_C_ID,
        MOCK_ITEM_GAS_ID, MOCK_ITEM_ORE_ID,
    },
    production_module_data::{
        ProductionModuleData, ProductionModuleId, ProductionModuleManifest,
        MOCK_PRODUCTION_MODULE_A_ID, MOCK_PRODUCTION_MODULE_B_ID, MOCK_PRODUCTION_MODULE_C_ID,
    },
    recipe_data::{
        RecipeData, RecipeElement, RecipeId, RecipeManifest, MOCK_RECIPE_A_ID, MOCK_RECIPE_B_ID,
        MOCK_RECIPE_C_ID,
    },
    ship_hull_data::{ShipHullData, ShipHullId, ShipHullManifest, MOCK_SHIP_HULL_A_ID},
    ship_weapon_data::{
        ShipWeaponData, ShipWeaponId, ShipWeaponManifest, GAS_COLLECTOR_ID, ORE_MINING_LASER_ID,
    },
    shipyard_module_data::{
        ShipyardModuleData, ShipyardModuleId, ShipyardModuleManifest, MOCK_SHIPYARD_MODULE_ID,
    },
};

/// A collection of all constant game data#
#[allow(dead_code)]
#[derive(SystemParam)]
pub struct GameData<'w> {
    pub items: Res<'w, ItemManifest>,
    pub item_recipes: Res<'w, RecipeManifest>,
    pub production_modules: Res<'w, ProductionModuleManifest>,
    pub ship_hulls: Res<'w, ShipHullManifest>,
    pub ship_weapons: Res<'w, ShipWeaponManifest>,
    pub shipyard_modules: Res<'w, ShipyardModuleManifest>,
    pub asteroids: Res<'w, AsteroidManifest>,
}

impl GameData<'_> {
    pub fn initialize_mock_data(world: &mut World) {
        let items = ItemManifest::from_mock_data(world);
        let item_recipes = RecipeManifest::from_mock_data(world);
        let production_modules = ProductionModuleManifest::from_mock_data(world);
        let ship_hulls = ShipHullManifest::from_mock_data(world);
        let ship_weapons = ShipWeaponManifest::from_mock_data(world);
        let shipyard_modules = ShipyardModuleManifest::from_mock_data(world);
        let asteroids = AsteroidManifest::from_mock_data(world);

        world.insert_resource(items);
        world.insert_resource(item_recipes);
        world.insert_resource(production_modules);
        world.insert_resource(ship_hulls);
        world.insert_resource(ship_weapons);
        world.insert_resource(shipyard_modules);
        world.insert_resource(asteroids);
    }
}

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
    asteroid_data::{AsteroidDataId, AsteroidManifest, CRYSTAL_ASTEROID_ID, IRON_ASTEROID_ID},
    item_data::{
        ItemData, ItemId, ItemManifest, CRYSTAL_ORE_ITEM_ID, HYDROGEN_ITEM_ID, IRON_ORE_ITEM_ID,
        REFINED_METALS_ITEM_ID, SILICA_ITEM_ID, WAFER_ITEM_ID,
    },
    production_module_data::{
        ProductionModuleData, ProductionModuleId, ProductionModuleManifest,
        REFINED_METALS_PRODUCTION_MODULE_ID, SILICA_PRODUCTION_MODULE_ID,
        WAFERS_PRODUCTION_MODULE_ID,
    },
    recipe_data::{
        RecipeData, RecipeElement, RecipeId, RecipeManifest, REFINED_METALS_RECIPE_ID,
        SILICA_RECIPE_ID, WAFERS_RECIPE_ID,
    },
    ship_hull_data::{
        ShipHullData, ShipHullId, ShipHullManifest, SHIP_HULL_MINER_ID, SHIP_HULL_TRANSPORT_ID,
    },
    ship_weapon_data::{
        ShipWeaponData, ShipWeaponId, ShipWeaponManifest, GAS_COLLECTOR_ID, ORE_MINING_LASER_ID,
    },
    shipyard_module_data::{
        ShipyardModuleData, ShipyardModuleId, ShipyardModuleManifest, MOCK_SHIPYARD_MODULE_ID,
    },
};

#[cfg(test)]
pub use item_data::{RawItemData, RawItemManifest};

/// An enum which differentiates between the various Modules that make up a station.
pub enum ConstructableModuleId {
    ProductionModule(ProductionModuleId),
    ShipyardModule(ShipyardModuleId),
}

/// A collection of all constant game data
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

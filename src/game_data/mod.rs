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
use serde::{Deserialize, Serialize};
#[allow(unused)]
pub use {
    asteroid_data::{AsteroidDataId, AsteroidManifest, CRYSTAL_ASTEROID_ID, IRON_ASTEROID_ID},
    item_data::{
        CRYSTAL_ORE_ITEM_ID, HYDROGEN_ITEM_ID, IRON_ORE_ITEM_ID, ItemData, ItemId, ItemManifest,
        REFINED_METALS_ITEM_ID, SILICA_ITEM_ID, WAFER_ITEM_ID,
    },
    production_module_data::{
        ProductionModuleData, ProductionModuleId, ProductionModuleManifest,
        REFINED_METALS_PRODUCTION_MODULE_ID, SILICA_PRODUCTION_MODULE_ID,
        WAFERS_PRODUCTION_MODULE_ID,
    },
    recipe_data::{
        REFINED_METALS_RECIPE_ID, RecipeData, RecipeElement, RecipeId, RecipeManifest,
        SILICA_RECIPE_ID, WAFERS_RECIPE_ID,
    },
    ship_hull_data::{
        SHIP_HULL_MINER_ID, SHIP_HULL_TRANSPORT_ID, ShipHullData, ShipHullId, ShipHullManifest,
    },
    ship_weapon_data::{
        CONSTRUCTION_TOOL_ID, GAS_COLLECTOR_ID, ORE_MINING_LASER_ID, ShipWeaponData, ShipWeaponId,
        ShipWeaponManifest,
    },
    shipyard_module_data::{
        MOCK_SHIPYARD_MODULE_ID, ShipyardModuleData, ShipyardModuleId, ShipyardModuleManifest,
    },
};

#[cfg(test)]
pub use item_data::{RawItemData, RawItemManifest};

/// An enum which differentiates between the various module kinds which make up a station.
#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
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

pub trait Constructable {
    fn get_constructable_data(&self) -> &ConstructableSiteData;
}

/// Contains common data necessary for everything that can be constructed in space.
#[derive(Deserialize)] // TODO: Not necessary once we implement raw constructable module data
pub struct ConstructableSiteData {
    /// The amount of build power necessary to build this module.
    pub required_build_power: u32,
    /// How much progress is necessary to advance between individual steps.
    pub progress_per_step: f32,
    /// The bill of materials required to build this module
    pub required_materials: Vec<RecipeElement>,
    /// required_materials split up into multiple steps.
    pub required_materials_per_step: Vec<Vec<RecipeElement>>,
}

impl ConstructableSiteData {
    pub fn new(required_build_power: u32, required_materials: Vec<RecipeElement>) -> Self {
        let step_count = 10; // TODO: Dynamically set step count depending on required build power
        let mut required_materials_per_step = Vec::default();
        for _ in 0..step_count {
            let mut step_materials = Vec::default();
            for ingredient in &required_materials {
                step_materials.push(RecipeElement {
                    item_id: ingredient.item_id,
                    amount: ingredient.amount / step_count, // TODO: handle fractions, right now every value needs to be divisible by step_count
                })
            }

            required_materials_per_step.push(step_materials);
        }

        Self {
            required_build_power,
            required_materials,
            required_materials_per_step,
            progress_per_step: required_build_power as f32 / step_count as f32,
        }
    }
}

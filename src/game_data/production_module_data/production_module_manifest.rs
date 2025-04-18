use crate::game_data::from_mock_data::FromMockData;
use crate::game_data::generic_manifest_without_raw_data::GenericManifestWithoutRawData;
use crate::game_data::production_module_data::{
    REFINED_METALS_PRODUCTION_MODULE_NAME, SILICA_PRODUCTION_MODULE_ID,
    SILICA_PRODUCTION_MODULE_NAME, WAFERS_PRODUCTION_MODULE_NAME,
};
use crate::game_data::{
    CRYSTAL_ORE_ITEM_ID, ConstructableSiteData, IRON_ORE_ITEM_ID, ProductionModuleData,
    REFINED_METALS_ITEM_ID, REFINED_METALS_PRODUCTION_MODULE_ID, REFINED_METALS_RECIPE_ID,
    RecipeElement, SILICA_ITEM_ID, SILICA_RECIPE_ID, WAFERS_PRODUCTION_MODULE_ID, WAFERS_RECIPE_ID,
};
use bevy::prelude::World;
use bevy::utils::HashMap;

/// Contains all parsed Production Modules.
pub type ProductionModuleManifest = GenericManifestWithoutRawData<ProductionModuleData>;

impl FromMockData for ProductionModuleManifest {
    #[must_use]
    fn from_mock_data(_world: &mut World) -> Self {
        let mock_modules = HashMap::from([
            (
                SILICA_PRODUCTION_MODULE_ID,
                ProductionModuleData {
                    id: SILICA_PRODUCTION_MODULE_ID,
                    name: SILICA_PRODUCTION_MODULE_NAME.into(),
                    available_recipes: vec![SILICA_RECIPE_ID],
                    constructable_data: ConstructableSiteData::new(
                        1000,
                        vec![RecipeElement {
                            item_id: CRYSTAL_ORE_ITEM_ID,
                            amount: 100,
                        }],
                    ),
                },
            ),
            (
                REFINED_METALS_PRODUCTION_MODULE_ID,
                ProductionModuleData {
                    id: REFINED_METALS_PRODUCTION_MODULE_ID,
                    name: REFINED_METALS_PRODUCTION_MODULE_NAME.into(),
                    available_recipes: vec![REFINED_METALS_RECIPE_ID],
                    constructable_data: ConstructableSiteData::new(
                        1000,
                        vec![RecipeElement {
                            item_id: IRON_ORE_ITEM_ID,
                            amount: 100,
                        }],
                    ),
                },
            ),
            (
                WAFERS_PRODUCTION_MODULE_ID,
                ProductionModuleData {
                    id: WAFERS_PRODUCTION_MODULE_ID,
                    name: WAFERS_PRODUCTION_MODULE_NAME.into(),
                    available_recipes: vec![WAFERS_RECIPE_ID],
                    constructable_data: ConstructableSiteData::new(
                        1000,
                        vec![
                            RecipeElement {
                                item_id: REFINED_METALS_ITEM_ID,
                                amount: 100,
                            },
                            RecipeElement {
                                item_id: SILICA_ITEM_ID,
                                amount: 42,
                            },
                        ],
                    ),
                },
            ),
        ]);

        Self::from(mock_modules)
    }
}

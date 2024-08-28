use crate::game_data::from_mock_data::FromMockData;
use crate::game_data::generic_manifest_without_raw_data::GenericManifestWithoutRawData;
use crate::game_data::production_module_data::MOCK_PRODUCTION_MODULE_A_ID;
use crate::game_data::{
    ProductionModuleData, MOCK_PRODUCTION_MODULE_B_ID, MOCK_PRODUCTION_MODULE_C_ID,
    MOCK_RECIPE_A_ID, MOCK_RECIPE_B_ID, MOCK_RECIPE_C_ID,
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
                MOCK_PRODUCTION_MODULE_A_ID,
                ProductionModuleData {
                    id: MOCK_PRODUCTION_MODULE_A_ID,
                    name: "Production Module A".to_string(),
                    available_recipes: vec![MOCK_RECIPE_A_ID],
                },
            ),
            (
                MOCK_PRODUCTION_MODULE_B_ID,
                ProductionModuleData {
                    id: MOCK_PRODUCTION_MODULE_B_ID,
                    name: "Production Module B".to_string(),
                    available_recipes: vec![MOCK_RECIPE_B_ID],
                },
            ),
            (
                MOCK_PRODUCTION_MODULE_C_ID,
                ProductionModuleData {
                    id: MOCK_PRODUCTION_MODULE_C_ID,
                    name: "Production Module C".to_string(),
                    available_recipes: vec![MOCK_RECIPE_C_ID],
                },
            ),
        ]);

        Self::from(mock_modules)
    }
}

use crate::game_data::from_mock_data::FromMockData;
use crate::game_data::production_module_data::MOCK_PRODUCTION_MODULE_A_ID;
use crate::game_data::{
    ProductionModuleData, ProductionModuleId, MOCK_PRODUCTION_MODULE_B_ID,
    MOCK_PRODUCTION_MODULE_C_ID, MOCK_RECIPE_A_ID, MOCK_RECIPE_B_ID, MOCK_RECIPE_C_ID,
};
use bevy::asset::Asset;
use bevy::prelude::{Resource, TypePath, World};
use bevy::utils::HashMap;
use leafwing_manifest::identifier::Id;
use leafwing_manifest::manifest::{Manifest, ManifestFormat};
use serde::Deserialize;

#[derive(Resource, Asset, TypePath, Deserialize)]
pub struct ProductionModuleManifest {
    productions: HashMap<ProductionModuleId, ProductionModuleData>,
}

impl ProductionModuleManifest {
    #[must_use]
    pub fn get_by_ref(&self, id: &ProductionModuleId) -> Option<&ProductionModuleData> {
        self.productions.get(id)
    }
}

impl FromMockData for ProductionModuleManifest {
    #[must_use]
    fn from_mock_data(world: &mut World) -> Self {
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
        Self::from_raw_manifest(
            ProductionModuleManifest {
                productions: mock_modules,
            },
            world,
        )
            .unwrap()
    }
}

impl Manifest for ProductionModuleManifest {
    type RawManifest = ProductionModuleManifest;
    type RawItem = ProductionModuleData;
    type Item = ProductionModuleData;
    type ConversionError = std::convert::Infallible;
    const FORMAT: ManifestFormat = ManifestFormat::Custom;

    fn from_raw_manifest(
        raw_manifest: Self::RawManifest,
        _world: &mut World,
    ) -> Result<Self, Self::ConversionError> {
        Ok(raw_manifest)
    }

    #[must_use]
    fn get(&self, id: Id<Self::Item>) -> Option<&Self::Item> {
        self.productions.get(&id)
    }
}

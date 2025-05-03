use crate::game_data::from_mock_data::FromMockData;
use crate::game_data::generic_manifest_without_raw_data::GenericManifestWithoutRawData;
use crate::game_data::shipyard_module_data::shipyard_module::ShipyardModuleData;
use crate::game_data::{
    ConstructableSiteData, MOCK_SHIPYARD_MODULE_ID, REFINED_METALS_ITEM_ID, RecipeElement,
};
use bevy::platform::collections::HashMap;
use bevy::prelude::World;

pub type ShipyardModuleManifest = GenericManifestWithoutRawData<ShipyardModuleData>;

impl FromMockData for ShipyardModuleManifest {
    #[must_use]
    fn from_mock_data(_world: &mut World) -> Self {
        let mock_modules = HashMap::from([(
            MOCK_SHIPYARD_MODULE_ID,
            ShipyardModuleData {
                id: MOCK_SHIPYARD_MODULE_ID,
                name: "Debug Shipyard".to_string(),
                constructable_data: ConstructableSiteData::new(
                    1000,
                    vec![RecipeElement {
                        item_id: REFINED_METALS_ITEM_ID,
                        amount: 500,
                    }],
                ),
            },
        )]);

        Self::from(mock_modules)
    }
}

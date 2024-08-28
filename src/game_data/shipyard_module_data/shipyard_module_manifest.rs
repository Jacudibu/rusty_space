use crate::game_data::from_mock_data::FromMockData;
use crate::game_data::generic_manifest_without_raw_data::GenericManifestWithoutRawData;
use crate::game_data::shipyard_module_data::shipyard_module::ShipyardModuleData;
use crate::game_data::MOCK_SHIPYARD_MODULE_ID;
use bevy::prelude::World;
use bevy::utils::HashMap;

pub type ShipyardModuleManifest = GenericManifestWithoutRawData<ShipyardModuleData>;

impl FromMockData for ShipyardModuleManifest {
    #[must_use]
    fn from_mock_data(_world: &mut World) -> Self {
        let mock_modules = HashMap::from([(
            MOCK_SHIPYARD_MODULE_ID,
            ShipyardModuleData {
                id: MOCK_SHIPYARD_MODULE_ID,
                name: "Debug Shipyard".to_string(),
            },
        )]);

        Self::from(mock_modules)
    }
}

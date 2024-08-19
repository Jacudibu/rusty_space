use crate::game_data::shipyard_module_data::shipyard_module::ShipyardModuleData;
use crate::game_data::{ShipyardModuleId, MOCK_SHIPYARD_MODULE_ID};
use bevy::asset::Asset;
use bevy::prelude::{Resource, TypePath, World};
use bevy::utils::HashMap;
use leafwing_manifest::identifier::Id;
use leafwing_manifest::manifest::{Manifest, ManifestFormat};
use serde::Deserialize;

#[derive(Resource, Asset, TypePath, Deserialize)]
pub struct ShipyardModuleManifest {
    shipyards: HashMap<ShipyardModuleId, ShipyardModuleData>,
}

impl ShipyardModuleManifest {
    #[must_use]
    pub fn get_by_ref(&self, id: &ShipyardModuleId) -> Option<&ShipyardModuleData> {
        self.shipyards.get(id)
    }

    #[must_use]
    pub fn from_mock_data(world: &mut World) -> Self {
        let mock_modules = HashMap::from([(
            MOCK_SHIPYARD_MODULE_ID,
            ShipyardModuleData {
                id: MOCK_SHIPYARD_MODULE_ID,
                name: "Debug Shipyard".to_string(),
            },
        )]);

        Self::from_raw_manifest(
            ShipyardModuleManifest {
                shipyards: mock_modules,
            },
            world,
        )
        .unwrap()
    }
}

impl Manifest for ShipyardModuleManifest {
    type RawManifest = ShipyardModuleManifest;
    type RawItem = ShipyardModuleData;
    type Item = ShipyardModuleData;
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
        self.shipyards.get(&id)
    }
}

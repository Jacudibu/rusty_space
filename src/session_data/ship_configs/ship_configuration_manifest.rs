use crate::game_data::{
    RecipeElement, ShipHullData, ShipHullManifest, MOCK_ITEM_ID_A, MOCK_ITEM_ID_B, MOCK_ITEM_ID_C,
    MOCK_SHIP_HULL_A_ID,
};
use crate::session_data::ship_configs::ship_configuration::ShipConfigurationParts;
use crate::session_data::ship_configs::versioned_id::VersionedId;
use crate::session_data::ship_configs::DEBUG_SHIP_CONFIG_NAME;
use crate::session_data::{
    ShipConfigId, ShipConfiguration, ShipConfigurationVersions, DEBUG_SHIP_CONFIG,
};
use bevy::asset::Asset;
use bevy::prelude::{Resource, TypePath, World};
use bevy::utils::HashMap;
use leafwing_manifest::identifier::Id;
use leafwing_manifest::manifest::{Manifest, ManifestFormat};
use serde::Deserialize;

#[derive(Resource, Asset, TypePath, Deserialize, Default)]
pub struct ShipConfigurationManifest {
    items: HashMap<Id<ShipConfigurationVersions>, ShipConfigurationVersions>,
}

impl ShipConfigurationManifest {
    #[must_use]
    pub fn get_by_id(&self, version: &ShipConfigId) -> Option<&ShipConfiguration> {
        self.items.get(&version.id)?.get_version(&version.version)
    }

    #[must_use]
    pub fn get_latest(&self, version: &ShipConfigId) -> Option<&ShipConfiguration> {
        Some(self.items.get(&version.id)?.latest())
    }

    #[must_use]
    pub fn mock_data(world: &mut World) -> Self {
        let hulls = world.get_resource::<ShipHullManifest>().unwrap();

        let mut mock_data = HashMap::new();
        let id = VersionedId::from_name(DEBUG_SHIP_CONFIG_NAME);
        mock_data.insert(
            id.id,
            ShipConfigurationVersions::new(ShipConfiguration::from(
                DEBUG_SHIP_CONFIG,
                "Fancy new ship".into(),
                ShipConfigurationParts {
                    hull: MOCK_SHIP_HULL_A_ID,
                },
                hulls,
            )),
        );

        Self::from_raw_manifest(ShipConfigurationManifest { items: mock_data }, world).unwrap()
    }
}

impl Manifest for ShipConfigurationManifest {
    type RawManifest = ShipConfigurationManifest;
    type RawItem = ShipConfigurationVersions;
    type Item = ShipConfigurationVersions;
    type ConversionError = std::convert::Infallible;
    const FORMAT: ManifestFormat = ManifestFormat::Custom;

    fn from_raw_manifest(
        raw_manifest: Self::RawManifest,
        _world: &mut World,
    ) -> Result<Self, Self::ConversionError> {
        Ok(Self {
            items: raw_manifest.items,
        })
    }

    #[must_use]
    fn get(&self, id: Id<Self::Item>) -> Option<&Self::Item> {
        self.items.get(&id)
    }
}

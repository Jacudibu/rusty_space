use crate::game_data::{
    RecipeElement, ShipHullData, ShipHullManifest, ShipWeaponManifest, GAS_COLLECTOR_ID,
    MOCK_ITEM_A_ID, MOCK_ITEM_B_ID, MOCK_ITEM_C_ID, MOCK_SHIP_HULL_A_ID, ORE_MINING_LASER_ID,
};
use crate::session_data::ship_configs::ship_configuration::ShipConfigurationParts;
use crate::session_data::ship_configs::versioned_id::VersionedId;
use crate::session_data::ship_configs::{
    MOCK_HARVESTING_SHIP_CONFIG_ID, MOCK_HARVESTING_SHIP_CONFIG_NAME, MOCK_MINING_SHIP_CONFIG_ID,
    MOCK_MINING_SHIP_CONFIG_NAME, MOCK_TRANSPORT_SHIP_CONFIG_ID, MOCK_TRANSPORT_SHIP_CONFIG_NAME,
};
use crate::session_data::{ShipConfigId, ShipConfiguration, ShipConfigurationVersions};
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
        let weapons = world.get_resource::<ShipWeaponManifest>().unwrap();

        let mut mock_data = HashMap::new();
        mock_data.insert(
            VersionedId::from_name(MOCK_TRANSPORT_SHIP_CONFIG_NAME).id,
            ShipConfigurationVersions::new(ShipConfiguration::from(
                MOCK_TRANSPORT_SHIP_CONFIG_ID,
                "Transport".into(),
                ShipConfigurationParts {
                    hull: MOCK_SHIP_HULL_A_ID,
                    weapons: vec![],
                },
                hulls,
                weapons,
            )),
        );

        mock_data.insert(
            VersionedId::from_name(MOCK_MINING_SHIP_CONFIG_NAME).id,
            ShipConfigurationVersions::new(ShipConfiguration::from(
                MOCK_MINING_SHIP_CONFIG_ID,
                "Miner".into(),
                ShipConfigurationParts {
                    hull: MOCK_SHIP_HULL_A_ID,
                    weapons: vec![ORE_MINING_LASER_ID, ORE_MINING_LASER_ID],
                },
                hulls,
                weapons,
            )),
        );

        mock_data.insert(
            VersionedId::from_name(MOCK_HARVESTING_SHIP_CONFIG_NAME).id,
            ShipConfigurationVersions::new(ShipConfiguration::from(
                MOCK_HARVESTING_SHIP_CONFIG_ID,
                "Harvester".into(),
                ShipConfigurationParts {
                    hull: MOCK_SHIP_HULL_A_ID,
                    weapons: vec![GAS_COLLECTOR_ID, GAS_COLLECTOR_ID],
                },
                hulls,
                weapons,
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

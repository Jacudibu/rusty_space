use crate::game_data::{
    ShipHullManifest, ShipWeaponManifest, GAS_COLLECTOR_ID, ORE_MINING_LASER_ID,
    SHIP_HULL_MINER_ID, SHIP_HULL_TRANSPORT_ID,
};
use crate::session_data::ship_configs::ship_configuration::ShipConfigurationParts;
use crate::session_data::ship_configs::versioned_id::VersionedId;
use crate::session_data::ship_configs::{
    version, MOCK_HARVESTING_SHIP_CONFIG_ID, MOCK_HARVESTING_SHIP_CONFIG_NAME,
    MOCK_MINING_SHIP_CONFIG_ID, MOCK_MINING_SHIP_CONFIG_NAME, MOCK_TRANSPORT_SHIP_CONFIG_ID,
    MOCK_TRANSPORT_SHIP_CONFIG_NAME,
};
use crate::session_data::{ShipConfigId, ShipConfiguration, ShipConfigurationVersions};
use bevy::asset::Asset;
use bevy::ecs::system::SystemState;
use bevy::prelude::{Assets, Event, EventWriter, Image, Res, ResMut, Resource, TypePath, World};
use bevy::utils::HashMap;
use leafwing_manifest::identifier::Id;
use leafwing_manifest::manifest::{Manifest, ManifestFormat};
use serde::Deserialize;

#[derive(Resource, Asset, TypePath, Deserialize, Default)]
pub struct ShipConfigurationManifest {
    items: HashMap<Id<ShipConfigurationVersions>, ShipConfigurationVersions>,
}

#[derive(Event)]
pub struct ShipConfigurationAddedEvent {
    pub(crate) id: ShipConfigId,
}

impl ShipConfigurationManifest {
    #[must_use]
    pub fn get_by_id(&self, version: &ShipConfigId) -> Option<&ShipConfiguration> {
        self.items.get(&version.id)?.get_version(&version.version)
    }

    #[must_use]
    #[allow(dead_code)]
    pub fn get_latest(&self, version: &ShipConfigId) -> Option<&ShipConfiguration> {
        Some(self.items.get(&version.id)?.latest())
    }

    /// Inserts a new [ShipConfiguration] into this collection.
    #[allow(dead_code)]
    pub fn insert_new(
        &mut self,
        name: &str,
        initial_configuration: ShipConfiguration,
        mut added_events: EventWriter<ShipConfigurationAddedEvent>,
    ) {
        let id = VersionedId::from_name(name).id;
        self.items
            .insert(id, ShipConfigurationVersions::new(initial_configuration));

        added_events.send(ShipConfigurationAddedEvent {
            id: ShipConfigId {
                id,
                version: version::INITIAL_VERSION,
            },
        });
    }

    #[must_use]
    pub fn mock_data(world: &mut World) -> Self {
        let mut system_state: SystemState<(
            ResMut<Assets<Image>>,
            Res<ShipHullManifest>,
            Res<ShipWeaponManifest>,
        )> = SystemState::new(world);

        let (mut image_assets, hulls, weapons) = system_state.get_mut(world);

        let mut mock_data = HashMap::new();
        mock_data.insert(
            VersionedId::from_name(MOCK_TRANSPORT_SHIP_CONFIG_NAME).id,
            ShipConfigurationVersions::new(ShipConfiguration::from(
                MOCK_TRANSPORT_SHIP_CONFIG_ID,
                "Transport".into(),
                ShipConfigurationParts {
                    hull: SHIP_HULL_TRANSPORT_ID,
                    weapons: vec![],
                },
                &hulls,
                &weapons,
                &mut image_assets,
            )),
        );

        mock_data.insert(
            VersionedId::from_name(MOCK_MINING_SHIP_CONFIG_NAME).id,
            ShipConfigurationVersions::new(ShipConfiguration::from(
                MOCK_MINING_SHIP_CONFIG_ID,
                "Miner".into(),
                ShipConfigurationParts {
                    hull: SHIP_HULL_MINER_ID,
                    weapons: vec![ORE_MINING_LASER_ID, ORE_MINING_LASER_ID],
                },
                &hulls,
                &weapons,
                &mut image_assets,
            )),
        );

        mock_data.insert(
            VersionedId::from_name(MOCK_HARVESTING_SHIP_CONFIG_NAME).id,
            ShipConfigurationVersions::new(ShipConfiguration::from(
                MOCK_HARVESTING_SHIP_CONFIG_ID,
                "Harvester".into(),
                ShipConfigurationParts {
                    hull: SHIP_HULL_MINER_ID,
                    weapons: vec![GAS_COLLECTOR_ID, GAS_COLLECTOR_ID],
                },
                &hulls,
                &weapons,
                &mut image_assets,
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
        world: &mut World,
    ) -> Result<Self, Self::ConversionError> {
        let mut result = Self::default();
        let mut events = Vec::new();

        for (id, configs) in raw_manifest.items {
            events.extend(configs.versions.iter().map(|(version, _)| {
                ShipConfigurationAddedEvent {
                    id: ShipConfigId {
                        id,
                        version: *version,
                    },
                }
            }));
            result.items.insert(id, configs);
        }

        world.send_event_batch(events);
        Ok(result)
    }

    #[must_use]
    fn get(&self, id: Id<Self::Item>) -> Option<&Self::Item> {
        self.items.get(&id)
    }
}

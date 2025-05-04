use crate::game_data::ShipHullId;
use crate::game_data::from_mock_data::FromMockData;
use crate::game_data::generic_manifest::GenericManifest;
use crate::game_data::ship_hull_data::ShipHullData;
use crate::game_data::ship_hull_data::raw_ship_hull::RawShipHullData;
use crate::game_data::ship_hull_data::raw_ship_hull_manifest::RawShipHullManifest;
use bevy::asset::AssetServer;
use bevy::platform::collections::HashMap;
use bevy::prelude::World;
use leafwing_manifest::identifier::Id;
use leafwing_manifest::manifest::{Manifest, ManifestFormat};

/// Contains all parsed Ship Hull Modules.
pub type ShipHullManifest = GenericManifest<ShipHullData>;

impl Manifest for ShipHullManifest {
    type RawManifest = RawShipHullManifest;
    type RawItem = RawShipHullData;
    type Item = ShipHullData;
    type ConversionError = std::convert::Infallible;
    const FORMAT: ManifestFormat = ManifestFormat::Custom; // We currently don't parse from filesystem

    fn from_raw_manifest(
        raw_manifest: Self::RawManifest,
        world: &mut World,
    ) -> Result<Self, Self::ConversionError> {
        let asset_server = world.resource::<AssetServer>();

        let items: HashMap<_, _> = raw_manifest
            .raw_data
            .into_iter()
            .map(|raw_item| {
                let id = ShipHullId::from_name(&raw_item.name);

                let data = ShipHullData {
                    id,
                    name: raw_item.name,
                    ship_size: raw_item.ship_size,
                    inventory_size: raw_item.inventory_size,
                    maneuverability: raw_item.maneuverability,
                    weapon_slots: raw_item.weapon_slots,
                    required_materials: raw_item.required_materials,
                    build_time: raw_item.build_time,
                    sprite: asset_server.load(raw_item.sprite),
                };

                (id, data)
            })
            .collect();

        Ok(Self::from(items))
    }

    #[must_use]
    #[inline]
    fn get(&self, id: Id<Self::Item>) -> Option<&Self::Item> {
        self.get_by_ref(&id)
    }
}

impl FromMockData for ShipHullManifest {
    fn from_mock_data(world: &mut World) -> Self {
        Self::from_raw_manifest(RawShipHullManifest::mock_data(), world).unwrap()
    }
}

use crate::game_data::from_mock_data::FromMockData;
use crate::game_data::generic_manifest::GenericManifest;
use crate::game_data::item_data::raw_item::RawItemData;
use crate::game_data::item_data::raw_item_manifest::RawItemManifest;
use crate::game_data::{ItemData, ItemId};
use crate::utils::PriceRange;
use bevy::asset::AssetServer;
use bevy::prelude::World;
use bevy::utils::HashMap;
use leafwing_manifest::identifier::Id;
use leafwing_manifest::manifest::{Manifest, ManifestFormat};

/// Contains all parsed item data, which will never change during gameplay.
pub type ItemManifest = GenericManifest<ItemData>;

impl FromMockData for ItemManifest {
    #[must_use]
    fn from_mock_data(world: &mut World) -> Self {
        Self::from_raw_manifest(RawItemManifest::mock_data(), world).unwrap()
    }
}

impl Manifest for ItemManifest {
    type RawManifest = RawItemManifest;
    type RawItem = RawItemData;
    type Item = ItemData;
    type ConversionError = std::convert::Infallible;
    const FORMAT: ManifestFormat = ManifestFormat::Custom; // We currently don't parse from filesystem

    fn from_raw_manifest(
        raw_manifest: Self::RawManifest,
        world: &mut World,
    ) -> Result<Self, Self::ConversionError> {
        let asset_server = world.resource::<AssetServer>();

        let items: HashMap<_, _> = raw_manifest
            .items
            .into_iter()
            .map(|raw_item| {
                let icon = asset_server.load(raw_item.icon);
                let id = ItemId::from_name(&raw_item.id);

                let item = ItemData {
                    id,
                    name: raw_item.id,
                    price: PriceRange::new(raw_item.price_min, raw_item.price_max),
                    icon,
                };

                (id, item)
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

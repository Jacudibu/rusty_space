use crate::game_data::item_data::raw_item::RawItem;
use crate::game_data::item_data::raw_item_manifest::RawItemManifest;
use crate::game_data::{Item, ItemId};
use crate::utils::PriceRange;
use bevy::asset::AssetServer;
use bevy::prelude::{Resource, World};
use bevy::utils::HashMap;
use leafwing_manifest::identifier::Id;
use leafwing_manifest::manifest::{Manifest, ManifestFormat};

/// Contains all item data, which will never change during gameplay.
#[derive(Resource)]
pub struct ItemManifest {
    items: HashMap<ItemId, Item>,
}

impl ItemManifest {
    #[must_use]
    pub fn get_from_ref(&self, id: &ItemId) -> Option<&Item> {
        self.items.get(id)
    }

    #[must_use]
    pub fn from_mock_data(world: &mut World) -> Self {
        Self::from_raw_manifest(RawItemManifest::mock_data(), world).unwrap()
    }
}

impl Manifest for ItemManifest {
    type RawManifest = RawItemManifest;
    type RawItem = RawItem;
    type Item = Item;
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

                let item = Item {
                    id,
                    name: raw_item.id,
                    price: PriceRange::new(raw_item.price_min, raw_item.price_max),
                    icon,
                };

                (id, item)
            })
            .collect();

        Ok(ItemManifest { items })
    }

    #[must_use]
    fn get(&self, id: Id<Self::Item>) -> Option<&Self::Item> {
        self.items.get(&id)
    }
}

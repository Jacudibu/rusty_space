use crate::utils::PriceRange;
use bevy::asset::AssetServer;
use bevy::prelude::{Asset, Handle, Image, Resource, TypePath, World};
use bevy::utils::HashMap;
use leafwing_manifest::identifier::Id;
use leafwing_manifest::manifest::{Manifest, ManifestFormat};
use serde::Deserialize;
use std::path::PathBuf;

pub type ItemId = Id<Item>;

const DEBUG_ITEM_STRING_A: &str = "item_a";
const DEBUG_ITEM_STRING_B: &str = "item_b";
const DEBUG_ITEM_STRING_C: &str = "item_c";

pub const DEBUG_ITEM_ID_A: ItemId = ItemId::from_name(DEBUG_ITEM_STRING_A);
pub const DEBUG_ITEM_ID_B: ItemId = ItemId::from_name(DEBUG_ITEM_STRING_B);
pub const DEBUG_ITEM_ID_C: ItemId = ItemId::from_name(DEBUG_ITEM_STRING_C);
pub const DEBUG_ITEM_ID_ORE: ItemId = DEBUG_ITEM_ID_A;
pub const DEBUG_ITEM_ID_GAS: ItemId = DEBUG_ITEM_ID_B;

/// Holds all relevant data for one specific item.
pub struct Item {
    pub id: ItemId,
    pub name: String, // Should be determined through i18n and not stored here to allow language switching without restarts (?).
    pub icon: Handle<Image>,
    pub price: PriceRange,
}

/// Contains all item data, which will never change during gameplay.
#[derive(Resource)]
pub struct ItemManifest {
    items: HashMap<ItemId, Item>,
}

#[derive(Deserialize)]
pub struct RawItem {
    pub id: String,
    pub icon: PathBuf,
    pub price_min: u32,
    pub price_max: u32,
}

/// Contains the raw, unprocessed item data.
#[derive(Asset, TypePath, Deserialize)]
pub struct RawItemManifest {
    items: Vec<RawItem>,
}

impl RawItemManifest {
    pub fn mock_data() -> Self {
        Self {
            items: vec![
                RawItem {
                    id: DEBUG_ITEM_STRING_A.into(),
                    icon: "ui_icons/items/a.png".into(),
                    price_min: 5,
                    price_max: 1000,
                },
                RawItem {
                    id: DEBUG_ITEM_STRING_B.into(),
                    icon: "ui_icons/items/b.png".into(),
                    price_min: 5,
                    price_max: 1000,
                },
                RawItem {
                    id: DEBUG_ITEM_STRING_C.into(),
                    icon: "ui_icons/items/c.png".into(),
                    price_min: 5,
                    price_max: 1000,
                },
            ],
        }
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

    fn get(&self, id: Id<Self::Item>) -> Option<&Self::Item> {
        self.items.get(&id)
    }
}

impl ItemManifest {
    pub fn get_from_ref(&self, id: &ItemId) -> Option<&Item> {
        self.items.get(id)
    }
}

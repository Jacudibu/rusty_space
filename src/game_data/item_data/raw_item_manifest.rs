use crate::game_data::item_data::raw_item::RawItemData;
use crate::game_data::item_data::{
    CRYSTAL_ORE_ITEM_NAME, HYDROGEN_ITEM_NAME, IRON_ORE_ITEM_NAME, REFINED_METALS_ITEM_NAME,
    SILICA_ITEM_NAME, WAFER_ITEM_NAME,
};
use bevy::asset::Asset;
use bevy::prelude::TypePath;
use serde::Deserialize;

/// Contains the raw, unprocessed item data.
#[derive(Asset, TypePath, Deserialize)]
pub struct RawItemManifest {
    pub(crate) items: Vec<RawItemData>,
}

impl RawItemManifest {
    pub fn mock_data() -> Self {
        Self {
            items: vec![
                RawItemData {
                    name: REFINED_METALS_ITEM_NAME.into(),
                    icon: "sprites/items/refined_metals.png".into(),
                    price_min: 5,
                    price_max: 1000,
                },
                RawItemData {
                    name: SILICA_ITEM_NAME.into(),
                    icon: "sprites/items/silica.png".into(),
                    price_min: 5,
                    price_max: 1000,
                },
                RawItemData {
                    name: WAFER_ITEM_NAME.into(),
                    icon: "sprites/items/wafer.png".into(),
                    price_min: 5,
                    price_max: 1000,
                },
                RawItemData {
                    name: IRON_ORE_ITEM_NAME.into(),
                    icon: "sprites/items/iron_ore.png".into(),
                    price_min: 5,
                    price_max: 1000,
                },
                RawItemData {
                    name: CRYSTAL_ORE_ITEM_NAME.into(),
                    icon: "sprites/items/crystal_ore.png".into(),
                    price_min: 5,
                    price_max: 1000,
                },
                RawItemData {
                    name: HYDROGEN_ITEM_NAME.into(),
                    icon: "sprites/items/hydrogen.png".into(),
                    price_min: 5,
                    price_max: 1000,
                },
            ],
        }
    }
}

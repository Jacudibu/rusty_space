use crate::game_data::item_data::raw_item::RawItem;
use crate::game_data::item_data::{MOCK_ITEM_STRING_A, MOCK_ITEM_STRING_B, MOCK_ITEM_STRING_C};
use bevy::asset::Asset;
use bevy::prelude::TypePath;
use serde::Deserialize;

/// Contains the raw, unprocessed item data.
#[derive(Asset, TypePath, Deserialize)]
pub struct RawItemManifest {
    pub(crate) items: Vec<RawItem>,
}

impl RawItemManifest {
    pub fn mock_data() -> Self {
        Self {
            items: vec![
                RawItem {
                    id: MOCK_ITEM_STRING_A.into(),
                    icon: "ui_icons/items/a.png".into(),
                    price_min: 5,
                    price_max: 1000,
                },
                RawItem {
                    id: MOCK_ITEM_STRING_B.into(),
                    icon: "ui_icons/items/b.png".into(),
                    price_min: 5,
                    price_max: 1000,
                },
                RawItem {
                    id: MOCK_ITEM_STRING_C.into(),
                    icon: "ui_icons/items/c.png".into(),
                    price_min: 5,
                    price_max: 1000,
                },
            ],
        }
    }
}

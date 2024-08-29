use crate::game_data::item_data::raw_item::RawItemData;
use crate::game_data::item_data::{MOCK_ITEM_A_NAME, MOCK_ITEM_B_NAME, MOCK_ITEM_C_NAME};
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
                    id: MOCK_ITEM_A_NAME.into(),
                    icon: "ui_icons/items/a.png".into(),
                    price_min: 5,
                    price_max: 1000,
                },
                RawItemData {
                    id: MOCK_ITEM_B_NAME.into(),
                    icon: "ui_icons/items/b.png".into(),
                    price_min: 5,
                    price_max: 1000,
                },
                RawItemData {
                    id: MOCK_ITEM_C_NAME.into(),
                    icon: "ui_icons/items/c.png".into(),
                    price_min: 5,
                    price_max: 1000,
                },
            ],
        }
    }
}

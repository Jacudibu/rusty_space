use crate::game_data::asteroid_data::raw_asteroid_data::RawAsteroidData;
use crate::game_data::asteroid_data::{MOCK_ASTEROID_NAME, SILICON_ASTEROID_NAME};
use crate::game_data::item_data::MOCK_ITEM_ORE_SILICON_ID;
use crate::game_data::MOCK_ITEM_ORE_ID;
use bevy::color::Color;
use bevy::prelude::{Asset, TypePath};
use serde::Deserialize;

#[derive(Asset, TypePath, Deserialize)]
pub struct RawAsteroidManifest {
    pub raw_data: Vec<RawAsteroidData>,
}

impl RawAsteroidManifest {
    pub fn mock_data() -> Self {
        Self {
            raw_data: vec![
                RawAsteroidData {
                    name: MOCK_ASTEROID_NAME.into(),
                    material: MOCK_ITEM_ORE_ID,
                    amount_min: 200,
                    amount_max: 500,
                    sprite: "asteroid.png".into(),
                    sprite_selected: "asteroid_selected.png".into(),
                    sprite_color: Color::WHITE,
                },
                RawAsteroidData {
                    name: SILICON_ASTEROID_NAME.into(),
                    material: MOCK_ITEM_ORE_SILICON_ID,
                    amount_min: 200,
                    amount_max: 500,
                    sprite: "asteroid_silicon.png".into(),
                    sprite_selected: "asteroid_silicon_selected.png".into(),
                    sprite_color: Color::WHITE,
                },
            ],
        }
    }
}

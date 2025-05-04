use crate::game_data::IRON_ORE_ITEM_ID;
use crate::game_data::asteroid_data::raw_asteroid_data::RawAsteroidData;
use crate::game_data::asteroid_data::{CRYSTAL_ASTEROID_NAME, IRON_ASTEROID_NAME};
use crate::game_data::item_data::CRYSTAL_ORE_ITEM_ID;
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
                    name: IRON_ASTEROID_NAME.into(),
                    material: IRON_ORE_ITEM_ID,
                    amount_min: 200,
                    amount_max: 500,
                    sprite: "sprites/asteroids/iron_asteroid.png".into(),
                    sprite_color: Color::WHITE,
                },
                RawAsteroidData {
                    name: CRYSTAL_ASTEROID_NAME.into(),
                    material: CRYSTAL_ORE_ITEM_ID,
                    amount_min: 200,
                    amount_max: 500,
                    sprite: "sprites/asteroids/crystal_asteroid.png".into(),
                    sprite_color: Color::WHITE,
                },
            ],
        }
    }
}

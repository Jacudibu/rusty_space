use crate::game_data::asteroid_data::raw_asteroid_data::RawAsteroidData;
use crate::game_data::MOCK_ITEM_ID_ORE;
use bevy::prelude::{Asset, TypePath};
use serde::Deserialize;

#[derive(Asset, TypePath, Deserialize)]
pub struct RawAsteroidManifest {
    pub raw_data: Vec<RawAsteroidData>,
}

impl RawAsteroidManifest {
    fn from_mock_data() -> Self {
        let raw_data = vec![RawAsteroidData {
            material: MOCK_ITEM_ID_ORE,
            amount_min: 200,
            amount_max: 500,
            sprite: "asteroid.png".into(),
        }];

        Self { raw_data }
    }
}

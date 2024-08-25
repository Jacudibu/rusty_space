use crate::game_data::from_mock_data::FromMockData;
use crate::game_data::ship_hull_data::{
    ShipHullData, ShipHullId, MOCK_SHIP_A_ID, MOCK_SHIP_A_NAME,
};
use crate::utils::ShipSize;
use bevy::asset::Asset;
use bevy::prelude::{Resource, TypePath, World};
use bevy::utils::HashMap;
use leafwing_manifest::identifier::Id;
use leafwing_manifest::manifest::{Manifest, ManifestFormat};
use serde::Deserialize;

#[derive(Resource, Asset, TypePath, Deserialize)]
pub struct ShipHullManifest {
    ship_hulls: HashMap<ShipHullId, ShipHullData>,
}

impl FromMockData for ShipHullManifest {
    fn from_mock_data(world: &mut World) -> Self {
        let mut mock_hulls = HashMap::new();

        mock_hulls.insert(
            MOCK_SHIP_A_ID,
            ShipHullData {
                id: MOCK_SHIP_A_ID,
                name: MOCK_SHIP_A_NAME.into(),
                ship_size: ShipSize::S,
                cargo_space: 500,
                required_materials: Vec::new(),
            },
        );

        Self::from_raw_manifest(
            ShipHullManifest {
                ship_hulls: mock_hulls,
            },
            world,
        )
            .unwrap()
    }
}

impl Manifest for ShipHullManifest {
    type RawManifest = ShipHullManifest;
    type RawItem = ShipHullData;
    type Item = ShipHullData;
    type ConversionError = std::convert::Infallible;
    const FORMAT: ManifestFormat = ManifestFormat::Custom;

    fn from_raw_manifest(
        raw_manifest: Self::RawManifest,
        _world: &mut World,
    ) -> Result<Self, Self::ConversionError> {
        Ok(ShipHullManifest {
            ship_hulls: raw_manifest.ship_hulls,
        })
    }

    fn get(&self, id: Id<Self::Item>) -> Option<&Self::Item> {
        self.ship_hulls.get(&id)
    }
}

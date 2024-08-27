use crate::game_data::from_mock_data::FromMockData;
use crate::game_data::ship_hull_data::ship_hull::ShipManeuverability;
use crate::game_data::ship_hull_data::{
    ShipHullData, ShipHullId, MOCK_SHIP_HULL_A_ID, MOCK_SHIP_HULL_A_NAME,
};
use crate::game_data::{RecipeElement, MOCK_ITEM_ID_A, MOCK_ITEM_ID_B, MOCK_ITEM_ID_C};
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

impl ShipHullManifest {
    #[inline]
    #[must_use]
    pub fn get_by_ref(&self, id: &ShipHullId) -> Option<&ShipHullData> {
        self.ship_hulls.get(id)
    }
}

impl FromMockData for ShipHullManifest {
    fn from_mock_data(world: &mut World) -> Self {
        let mut mock_hulls = HashMap::new();

        mock_hulls.insert(
            MOCK_SHIP_HULL_A_ID,
            ShipHullData {
                id: MOCK_SHIP_HULL_A_ID,
                name: MOCK_SHIP_HULL_A_NAME.into(),
                ship_size: ShipSize::S,
                inventory_size: 100,
                build_time: 5000,
                maneuverability: ShipManeuverability {
                    max_speed: 100.0,
                    acceleration: 10.0,
                    deceleration: 30.0,
                    max_angular_speed: 1.0,
                    angular_acceleration: 1.0,
                },
                required_materials: vec![
                    RecipeElement {
                        item_id: MOCK_ITEM_ID_A,
                        amount: 50,
                    },
                    RecipeElement {
                        item_id: MOCK_ITEM_ID_B,
                        amount: 23,
                    },
                    RecipeElement {
                        item_id: MOCK_ITEM_ID_C,
                        amount: 74,
                    },
                ],
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
        Ok(Self {
            ship_hulls: raw_manifest.ship_hulls,
        })
    }

    fn get(&self, id: Id<Self::Item>) -> Option<&Self::Item> {
        self.ship_hulls.get(&id)
    }
}

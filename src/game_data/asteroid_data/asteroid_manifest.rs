use crate::game_data::asteroid_data::asteroid_data_struct::AsteroidData;
use crate::game_data::asteroid_data::raw_asteroid_data::RawAsteroidData;
use crate::game_data::asteroid_data::raw_asteroid_manifest::RawAsteroidManifest;
use crate::game_data::asteroid_data::AsteroidDataId;
use crate::game_data::from_mock_data::FromMockData;
use crate::game_data::generic_manifest::GenericManifest;
use bevy::prelude::{AssetServer, World};
use bevy::utils::HashMap;
use leafwing_manifest::identifier::Id;
use leafwing_manifest::manifest::{Manifest, ManifestFormat};

/// Contains the parsed definitions and spawn instructions for all kinds of asteroids.
pub type AsteroidManifest = GenericManifest<AsteroidData>;

impl FromMockData for AsteroidManifest {
    #[must_use]
    fn from_mock_data(world: &mut World) -> Self {
        Self::from_raw_manifest(RawAsteroidManifest::mock_data(), world).unwrap()
    }
}

impl Manifest for AsteroidManifest {
    type RawManifest = RawAsteroidManifest;
    type RawItem = RawAsteroidData;
    type Item = AsteroidData;
    type ConversionError = std::convert::Infallible;
    const FORMAT: ManifestFormat = ManifestFormat::Custom;

    fn from_raw_manifest(
        raw_manifest: Self::RawManifest,
        world: &mut World,
    ) -> Result<Self, Self::ConversionError> {
        let asset_server = world.resource::<AssetServer>();

        let items: HashMap<_, _> = raw_manifest
            .raw_data
            .into_iter()
            .map(|raw_item| {
                let id = AsteroidDataId::from_name(&raw_item.name);

                let item = AsteroidData {
                    name: raw_item.name,
                    material: raw_item.material,
                    amount: raw_item.amount_min..raw_item.amount_max,
                    sprite: asset_server.load(raw_item.sprite),
                    sprite_selected: asset_server.load(raw_item.sprite_selected),
                    sprite_color: raw_item.sprite_color,
                };

                (id, item)
            })
            .collect();

        Ok(Self::from(items))
    }

    #[must_use]
    #[inline]
    fn get(&self, id: Id<Self::Item>) -> Option<&Self::Item> {
        self.get_by_ref(&id)
    }
}

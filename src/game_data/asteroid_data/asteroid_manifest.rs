use crate::game_data::asteroid_data::asteroid_data_struct::AsteroidData;
use crate::game_data::asteroid_data::raw_asteroid_data::RawAsteroidData;
use crate::game_data::asteroid_data::raw_asteroid_manifest::RawAsteroidManifest;
use crate::game_data::asteroid_data::AsteroidDataId;
use crate::game_data::generic_manifest::GenericManifest;
use bevy::prelude::{AssetServer, World};
use bevy::utils::HashMap;
use leafwing_manifest::identifier::Id;
use leafwing_manifest::manifest::{Manifest, ManifestFormat};

/// Contains the parsed definitions and spawn instructions for all kinds of asteroids.
type AsteroidManifest = GenericManifest<AsteroidData>;

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
                let item = AsteroidData {
                    material: raw_item.material,
                    amount: raw_item.amount_min..raw_item.amount_max,
                    sprite: asset_server.load(raw_item.sprite),
                };

                (AsteroidDataId::from_raw(item.material.raw()), item)
            })
            .collect();

        Ok(Self::from(items))
    }

    fn get(&self, id: Id<Self::Item>) -> Option<&Self::Item> {
        self.get_by_ref(&id)
    }
}

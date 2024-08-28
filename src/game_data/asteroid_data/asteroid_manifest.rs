use crate::game_data::asteroid_data::asteroid_data_struct::AsteroidData;
use crate::game_data::asteroid_data::raw_asteroid_data::RawAsteroidData;
use crate::game_data::asteroid_data::raw_asteroid_manifest::RawAsteroidManifest;
use crate::game_data::asteroid_data::AsteroidDataId;
use bevy::prelude::{AssetServer, Resource, World};
use bevy::utils::HashMap;
use leafwing_manifest::identifier::Id;
use leafwing_manifest::manifest::{Manifest, ManifestFormat};

#[derive(Resource)]
pub struct AsteroidManifest {
    items: HashMap<AsteroidDataId, AsteroidData>,
}

impl AsteroidManifest {
    #[must_use]
    #[inline]
    pub fn get_from_ref(&self, id: &AsteroidDataId) -> Option<&AsteroidData> {
        self.items.get(id)
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
                let item = AsteroidData {
                    material: raw_item.material,
                    amount: raw_item.amount_min..raw_item.amount_max,
                    sprite: asset_server.load(raw_item.sprite),
                };

                (AsteroidDataId::from_raw(item.material.raw()), item)
            })
            .collect();

        Ok(Self { items })
    }

    fn get(&self, id: Id<Self::Item>) -> Option<&Self::Item> {
        self.items.get(&id)
    }
}

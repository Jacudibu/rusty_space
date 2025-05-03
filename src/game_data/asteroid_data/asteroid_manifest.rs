use crate::game_data::asteroid_data::AsteroidDataId;
use crate::game_data::asteroid_data::asteroid_data_struct::AsteroidData;
use crate::game_data::asteroid_data::raw_asteroid_data::RawAsteroidData;
use crate::game_data::asteroid_data::raw_asteroid_manifest::RawAsteroidManifest;
use crate::game_data::from_mock_data::FromMockData;
use crate::game_data::generic_manifest::GenericManifest;
use crate::image_generator;
use bevy::asset::Assets;
use bevy::ecs::system::SystemState;
use bevy::image::Image;
use bevy::platform::collections::HashMap;
use bevy::prelude::{AssetServer, Res, ResMut, World};
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
        let mut system_state: SystemState<(ResMut<Assets<Image>>, Res<AssetServer>)> =
            SystemState::new(world);

        let (mut image_assets, asset_server) = system_state.get_mut(world);

        let items: HashMap<_, _> = raw_manifest
            .raw_data
            .into_iter()
            .map(|raw_item| {
                let id = AsteroidDataId::from_name(&raw_item.name);

                let item = AsteroidData {
                    name: raw_item.name,
                    material: raw_item.material,
                    amount: raw_item.amount_min..raw_item.amount_max,
                    sprite_selected:
                        image_generator::generate_image_with_highlighted_corners_from_asset_path(
                            &raw_item.sprite,
                            &mut image_assets,
                        ),
                    sprite: asset_server.load(raw_item.sprite),
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

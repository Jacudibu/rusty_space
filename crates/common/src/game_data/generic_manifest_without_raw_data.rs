use bevy::asset::Asset;
use bevy::platform::collections::HashMap;
use bevy::prelude::{Resource, TypePath, World};
use leafwing_manifest::identifier::Id;
use leafwing_manifest::manifest::{Manifest, ManifestFormat};
use serde::Deserialize;

/// Marker trait necessary for [`GenericManifestWithoutRawData`]
pub trait DataCanBeUsedAsRawData: Sized + Send + Sync + TypePath {}

/// Creating types based on this avoids writing the same boilerplate Manifest code for things which don't require intermediate data objects yet.
/// Once those are necessary, switch to [`GenericManifest`] and write a custom implementation for the Manifest trait.
///
/// Caveat: [`Data`] objects need to implement [`DataCanBeUsedAsRawData`] trait to make the compiler happy.
#[derive(Resource, Asset, TypePath, Deserialize)]
pub struct GenericManifestWithoutRawData<Data: DataCanBeUsedAsRawData> {
    items: HashMap<Id<Data>, Data>,
}

impl<Data: DataCanBeUsedAsRawData> GenericManifestWithoutRawData<Data> {
    #[must_use]
    #[inline]
    pub fn get_by_ref(&self, id: &Id<Data>) -> Option<&Data> {
        self.items.get(id)
    }
}

impl<Data: DataCanBeUsedAsRawData> From<HashMap<Id<Data>, Data>>
    for GenericManifestWithoutRawData<Data>
{
    #[must_use]
    fn from(value: HashMap<Id<Data>, Data>) -> Self {
        Self { items: value }
    }
}

impl<Data: DataCanBeUsedAsRawData + for<'de> Deserialize<'de>> Manifest
    for GenericManifestWithoutRawData<Data>
{
    type RawManifest = GenericManifestWithoutRawData<Data>;
    type RawItem = Data;
    type Item = Data;
    type ConversionError = std::convert::Infallible;
    const FORMAT: ManifestFormat = ManifestFormat::Custom;

    fn from_raw_manifest(
        raw_manifest: Self::RawManifest,
        _world: &mut World,
    ) -> Result<Self, Self::ConversionError> {
        Ok(Self {
            items: raw_manifest.items,
        })
    }

    fn get(&self, id: Id<Self::Item>) -> Option<&Self::Item> {
        self.get_by_ref(&id)
    }
}

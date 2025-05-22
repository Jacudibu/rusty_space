use bevy::platform::collections::HashMap;
use bevy::platform::collections::hash_map::Iter;
use bevy::prelude::Resource;
use leafwing_manifest::identifier::Id;
use std::ops::Index;

#[derive(Resource)]
pub struct GenericManifest<Data> {
    items: HashMap<Id<Data>, Data>,
}

impl<Data> GenericManifest<Data> {
    #[must_use]
    #[inline]
    pub fn get_by_ref(&self, id: &Id<Data>) -> Option<&Data> {
        self.items.get(id)
    }

    #[must_use]
    #[inline]
    pub fn iter(&self) -> Iter<'_, Id<Data>, Data> {
        self.items.iter()
    }
}

impl<Data> From<HashMap<Id<Data>, Data>> for GenericManifest<Data> {
    fn from(value: HashMap<Id<Data>, Data>) -> Self {
        Self { items: value }
    }
}

impl<Data> Index<&Id<Data>> for GenericManifest<Data> {
    type Output = Data;

    fn index(&self, index: &Id<Data>) -> &Self::Output {
        &self.items[index]
    }
}

impl<Data> Index<Id<Data>> for GenericManifest<Data> {
    type Output = Data;

    fn index(&self, index: Id<Data>) -> &Self::Output {
        &self.items[&index]
    }
}

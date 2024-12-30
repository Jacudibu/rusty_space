use bevy::prelude::Resource;
use bevy::utils::hashbrown::hash_map::Iter;
use bevy::utils::HashMap;
use leafwing_manifest::identifier::Id;

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
    #[must_use]
    fn from(value: HashMap<Id<Data>, Data>) -> Self {
        Self { items: value }
    }
}

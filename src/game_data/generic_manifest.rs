use bevy::prelude::Resource;
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
}

impl<Data> From<HashMap<Id<Data>, Data>> for GenericManifest<Data> {
    #[must_use]
    fn from(value: HashMap<Id<Data>, Data>) -> Self {
        Self { items: value }
    }
}

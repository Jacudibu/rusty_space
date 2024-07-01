use bevy::prelude::Resource;
use bevy::utils::HashMap;
use std::ops::{Deref, DerefMut};

/// Generic Resource type just containing a [HashMap] and implementing [Deref] and [DerefMut] for easy access.
#[derive(Resource)]
pub struct KeyValueResource<TKey, TValue> {
    inner: HashMap<TKey, TValue>,
}

impl<TKey, TValue> Deref for KeyValueResource<TKey, TValue> {
    type Target = HashMap<TKey, TValue>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<TKey, TValue> DerefMut for KeyValueResource<TKey, TValue> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<TKey, TValue> Default for KeyValueResource<TKey, TValue> {
    fn default() -> Self {
        KeyValueResource {
            inner: HashMap::new(),
        }
    }
}

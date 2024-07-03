use bevy::prelude::{Component, Entity};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

/// Wrapper around [Entity] to guarantee type safety when storing Entities for specific component combinations.
pub struct TypedEntity<T: Component>(Entity, PhantomData<T>);

impl<T: Component> TypedEntity<T> {
    pub fn get(&self) -> Entity {
        self.0
    }
}

impl<T: Component> From<Entity> for TypedEntity<T> {
    fn from(value: Entity) -> Self {
        Self(value, PhantomData)
    }
}

impl<T: Component> Clone for TypedEntity<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Component> Copy for TypedEntity<T> {}

impl<T: Component> PartialEq<Self> for TypedEntity<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: Component> Eq for TypedEntity<T> {}
impl<T: Component> Hash for TypedEntity<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}
